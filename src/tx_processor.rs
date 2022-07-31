use std::cmp::Ordering;

use crate::{
    model::{Account, Chargeback, Deposit, Dispute, Resolve, Transaction, Withdrawal},
    Error, TxBlockStore,
};

/// Processes transactions for an account.
#[derive(Debug)]
pub struct TxProcessor<'block_store> {
    /// Stores transactions.
    block_store: &'block_store TxBlockStore,
}

impl<'block_store> TxProcessor<'block_store> {
    /// Returns a new `TxProcessor`.
    pub fn new(block_store: &'block_store TxBlockStore) -> Self {
        Self { block_store }
    }

    /// Processes a transaction for an account.
    pub async fn process(
        &self,
        account: &mut Account,
        transaction: Transaction,
    ) -> Result<(), Error> {
        if account.locked() {
            // Don't process locked accounts.
            return Ok(());
        }

        match transaction {
            Transaction::Deposit(deposit) => self.handle_deposit(account, deposit),
            Transaction::Withdrawal(withdrawal) => self.handle_withdrawal(account, withdrawal),
            Transaction::Dispute(dispute) => self.handle_dispute(account, dispute).await,
            Transaction::Resolve(resolve) => self.handle_resolve(account, resolve).await,
            Transaction::Chargeback(chargeback) => {
                self.handle_chargeback(account, chargeback).await
            }
        }
    }

    fn handle_deposit(&self, account: &mut Account, deposit: Deposit) -> Result<(), Error> {
        let client = account.client();
        let tx = deposit.tx();
        let deposit_amount = deposit.amount();
        let available = account.available();
        let available_next = available.checked_add(deposit_amount);

        if deposit_amount.is_sign_negative() {
            return Err(Error::DepositAmountNegative {
                client,
                tx,
                amount: deposit_amount,
            });
        }

        available_next
            .ok_or(Error::DepositAvailableOverflow { client, tx })
            .and_then(|available_next| {
                Account::try_new(
                    client,
                    available_next,
                    account.held(),
                    account.locked(),
                    account.disputed_txs().to_vec(),
                )
                .map_err(|_| Error::DepositTotalOverflow { client, tx })
            })
            .map(|account_updated| *account = account_updated)
    }

    fn handle_withdrawal(
        &self,
        account: &mut Account,
        withdrawal: Withdrawal,
    ) -> Result<(), Error> {
        let client = account.client();
        let tx = withdrawal.tx();
        let withdrawal_amount = withdrawal.amount();
        let available = account.available();

        if withdrawal_amount.is_sign_negative() {
            Err(Error::WithdrawalAmountNegative {
                client,
                tx,
                amount: withdrawal_amount,
            })
        } else if withdrawal_amount.cmp(&available) == Ordering::Greater {
            // Not enough amount, don't change the account values.
            Ok(())
        } else {
            let available_next = available.saturating_sub(withdrawal.amount());
            let account_updated = Account::try_new(
                client,
                available_next,
                account.held(),
                account.locked(),
                account.disputed_txs().to_vec(),
            )
            .expect(
                "Overflow impossible: Withdrawal amount is less than or equal to available amount, \
                        and is non-negative.",
            );
            *account = account_updated;

            Ok(())
        }
    }

    async fn handle_dispute(&self, account: &mut Account, dispute: Dispute) -> Result<(), Error> {
        if account.client() != dispute.client() {
            // Only allow a client to dispute transactions to their own account.
            return Ok(());
        }

        let transaction = self.block_store.find_transaction(dispute.tx()).await;
        match transaction {
            Ok(transaction) => {
                let (tx, amount) = match transaction {
                    Transaction::Deposit(deposit) => {
                        let tx = deposit.tx();
                        let amount = deposit.amount();
                        (tx, amount)
                    }
                    _ => unreachable!(
                        "Only deposits may be disputed -- see `TxBlockStore::find_transaction`."
                    ),
                };

                let client = account.client();
                let available = account.available();
                let held = account.held();

                if amount.cmp(&available) == Ordering::Greater {
                    // Not enough available to hold.
                    return Err(Error::DisputeInsufficientAvailable {
                        client,
                        tx,
                        available,
                        amount,
                    });
                }

                // never negative, as we've done the comparison above
                let available_next = available.saturating_sub(amount);

                let held_next = held.checked_add(amount).ok_or(Error::DisputeHeldOverflow {
                    client,
                    tx,
                    held,
                    amount,
                })?;

                let mut disputed_txs = Vec::with_capacity(account.disputed_txs().len() + 1);
                disputed_txs.extend_from_slice(account.disputed_txs());
                disputed_txs.push(tx);
                let account_updated = Account::try_new(
                    client,
                    available_next,
                    held_next,
                    account.locked(),
                    disputed_txs,
                )
                .expect(
                    "Overflow impossible: available and held amounts should equal previous total.",
                );

                *account = account_updated;

                Ok(())
            }
            Err(Error::DisputeTxNotFound { .. }) => {
                // Safe to ignore, according to spec
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    async fn handle_resolve(&self, account: &mut Account, resolve: Resolve) -> Result<(), Error> {
        if account.client() != resolve.client() {
            // Only allow a client to dispute transactions to their own account.
            return Ok(());
        }

        let resolve_tx = resolve.tx();
        let disputed_tx_pos = account
            .disputed_txs()
            .iter()
            .position(|disputed_tx| disputed_tx == &resolve_tx);

        if let Some(disputed_tx_pos) = disputed_tx_pos {
            let transaction = self.block_store.find_transaction(resolve_tx).await;
            match transaction {
                Ok(transaction) => {
                    let (tx, amount) = match transaction {
                        Transaction::Deposit(deposit) => {
                            let tx = deposit.tx();
                            let amount = deposit.amount();
                            (tx, amount)
                        }
                        _ => unreachable!(
                            "Only deposits may be disputed -- see `TxBlockStore::find_transaction`."
                        ),
                    };

                    let client = account.client();
                    let available = account.available();
                    let held = account.held();

                    if amount.cmp(&held) == Ordering::Greater {
                        // Not enough held to subtract.
                        return Err(Error::ResolveInsufficientHeld {
                            client,
                            tx,
                            held,
                            amount,
                        });
                    }

                    // never negative, as we've done the comparison above
                    let held_next = held.saturating_sub(amount);

                    let available_next =
                        available
                            .checked_add(amount)
                            .ok_or(Error::ResolveAvailableOverflow {
                                client,
                                tx,
                                available,
                                amount,
                            })?;

                    let mut disputed_txs = account.disputed_txs().to_vec();
                    disputed_txs.swap_remove(disputed_tx_pos);
                    let account_updated = Account::try_new(
                        client,
                        available_next,
                        held_next,
                        account.locked(),
                        disputed_txs,
                    )
                    .expect("Overflow impossible: available and held amounts should equal previous total.");

                    *account = account_updated;

                    Ok(())
                }
                Err(Error::DisputeTxNotFound { .. }) => {
                    // Safe to ignore, according to spec
                    Ok(())
                }
                Err(e) => Err(e),
            }
        } else {
            // We can ignore this, according to spec.
            Ok(())
        }
    }

    async fn handle_chargeback(
        &self,
        account: &mut Account,
        chargeback: Chargeback,
    ) -> Result<(), Error> {
        if account.client() != chargeback.client() {
            // Only allow a client to dispute transactions to their own account.
            return Ok(());
        }

        let chargeback_tx = chargeback.tx();
        let disputed_tx_pos = account
            .disputed_txs()
            .iter()
            .position(|disputed_tx| disputed_tx == &chargeback_tx);

        if let Some(disputed_tx_pos) = disputed_tx_pos {
            let transaction = self.block_store.find_transaction(chargeback_tx).await;
            match transaction {
                Ok(transaction) => {
                    let (tx, amount) = match transaction {
                        Transaction::Deposit(deposit) => {
                            let tx = deposit.tx();
                            let amount = deposit.amount();
                            (tx, amount)
                        }
                        _ => unreachable!(
                            "Only deposits may be disputed -- see `TxBlockStore::find_transaction`."
                        ),
                    };

                    let client = account.client();
                    let available = account.available();
                    let held = account.held();

                    if amount.cmp(&held) == Ordering::Greater {
                        // Not enough held to subtract.
                        return Err(Error::ChargebackInsufficientHeld {
                            client,
                            tx,
                            held,
                            amount,
                        });
                    }

                    // never negative, as we've done the comparison above
                    let held_next = held.saturating_sub(amount);

                    let mut disputed_txs = account.disputed_txs().to_vec();
                    disputed_txs.swap_remove(disputed_tx_pos);
                    let account_updated = Account::try_new(
                        client,
                        available,
                        held_next,
                        true,
                        disputed_txs,
                    )
                    .expect("Overflow impossible: available and held amounts should be less than previous total.");

                    *account = account_updated;

                    Ok(())
                }
                Err(Error::DisputeTxNotFound { .. }) => {
                    // Safe to ignore, according to spec
                    Ok(())
                }
                Err(e) => Err(e),
            }
        } else {
            // We can ignore this, according to spec.
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;

    use super::TxProcessor;
    use crate::{
        model::{Account, ClientId, Deposit, TxId, Withdrawal},
        Error, TxBlockStore,
    };

    #[test]
    fn deposit_positive_amount_adds_available_amount() -> Result<(), Error> {
        let client = ClientId::new(1);
        let tx = TxId::new(2);
        let amount = dec!(1.0);
        let mut account = Account::empty(client);

        let tx_block_store = &TxBlockStore::try_new().expect("Failed to initialize block store.");
        let tx_processor = TxProcessor::new(tx_block_store);
        tx_processor.handle_deposit(&mut account, Deposit::new(client, tx, amount))?;

        let account_expected = Account::try_new(client, dec!(1.0), dec!(0.0), false, vec![])
            .expect("Test data invalid.");
        assert_eq!(account_expected, account);
        Ok(())
    }

    #[test]
    fn deposit_negative_amount_returns_err() -> Result<(), Error> {
        let client = ClientId::new(1);
        let tx = TxId::new(2);
        let amount = dec!(-1.0);
        let mut account = Account::empty(client);

        let tx_block_store = &TxBlockStore::try_new().expect("Failed to initialize block store.");
        let tx_processor = TxProcessor::new(tx_block_store);
        let result = tx_processor.handle_deposit(&mut account, Deposit::new(client, tx, amount));

        assert!(matches!(
            result,
            Err(Error::DepositAmountNegative { client, tx, amount })
            if client == ClientId::new(1) && tx == TxId::new(2) && amount == dec!(-1.0)
        ));
        Ok(())
    }

    #[test]
    fn deposit_amount_overflow_available() -> Result<(), Error> {
        let client = ClientId::new(1);
        let tx = TxId::new(2);
        let amount = Decimal::MAX;
        let mut account = Account::try_new(client, dec!(1.0), dec!(0.0), false, vec![])
            .expect("Test data invalid.");

        let tx_block_store = &TxBlockStore::try_new().expect("Failed to initialize block store.");
        let tx_processor = TxProcessor::new(tx_block_store);
        let result = tx_processor.handle_deposit(&mut account, Deposit::new(client, tx, amount));

        assert!(matches!(
            result,
            Err(Error::DepositAvailableOverflow { client, tx })
            if client == ClientId::new(1) && tx == TxId::new(2)
        ));
        Ok(())
    }

    #[test]
    fn deposit_total_overflow() -> Result<(), Error> {
        let client = ClientId::new(1);
        let tx = TxId::new(2);
        let amount = Decimal::MAX.saturating_sub(dec!(1.0));
        let mut account = Account::try_new(client, dec!(1.0), dec!(2.0), false, vec![])
            .expect("Test data invalid.");

        let tx_block_store = &TxBlockStore::try_new().expect("Failed to initialize block store.");
        let tx_processor = TxProcessor::new(tx_block_store);
        let result = tx_processor.handle_deposit(&mut account, Deposit::new(client, tx, amount));

        assert!(matches!(
            result,
            Err(Error::DepositTotalOverflow { client, tx })
            if client == ClientId::new(1) && tx == TxId::new(2)
        ));
        Ok(())
    }

    #[test]
    fn withdrawal_positive_amount_with_sufficient_extra_amount_subtracts_amount()
    -> Result<(), Error> {
        let client = ClientId::new(1);
        let tx = TxId::new(2);
        let amount = dec!(1.0);
        let mut account = Account::try_new(client, dec!(2.0), dec!(0.0), false, vec![])
            .expect("Test data invalid.");

        let tx_block_store = &TxBlockStore::try_new().expect("Failed to initialize block store.");
        let tx_processor = TxProcessor::new(tx_block_store);
        tx_processor.handle_withdrawal(&mut account, Withdrawal::new(client, tx, amount))?;

        let account_expected = Account::try_new(client, dec!(1.0), dec!(0.0), false, vec![])
            .expect("Test data invalid.");
        assert_eq!(account_expected, account);
        Ok(())
    }

    #[test]
    fn withdrawal_positive_amount_with_sufficient_exact_amount_subtracts_amount()
    -> Result<(), Error> {
        let client = ClientId::new(1);
        let tx = TxId::new(2);
        let amount = dec!(1.0);
        let mut account = Account::try_new(client, dec!(1.0), dec!(0.0), false, vec![])
            .expect("Test data invalid.");

        let tx_block_store = &TxBlockStore::try_new().expect("Failed to initialize block store.");
        let tx_processor = TxProcessor::new(tx_block_store);
        tx_processor.handle_withdrawal(&mut account, Withdrawal::new(client, tx, amount))?;

        let account_expected = Account::try_new(client, dec!(0.0), dec!(0.0), false, vec![])
            .expect("Test data invalid.");
        assert_eq!(account_expected, account);
        Ok(())
    }

    #[test]
    fn withdrawal_positive_amount_with_insufficient_amount_does_nothing() -> Result<(), Error> {
        let client = ClientId::new(1);
        let tx = TxId::new(2);
        let amount = dec!(2.0);
        let mut account = Account::try_new(client, dec!(1.0), dec!(0.0), false, vec![])
            .expect("Test data invalid.");

        let tx_block_store = &TxBlockStore::try_new().expect("Failed to initialize block store.");
        let tx_processor = TxProcessor::new(tx_block_store);
        tx_processor.handle_withdrawal(&mut account, Withdrawal::new(client, tx, amount))?;

        let account_expected = Account::try_new(client, dec!(1.0), dec!(0.0), false, vec![])
            .expect("Test data invalid.");
        assert_eq!(account_expected, account);
        Ok(())
    }

    #[test]
    fn withdrawal_negative_amount_returns_err() -> Result<(), Error> {
        let client = ClientId::new(1);
        let tx = TxId::new(2);
        let amount = dec!(-1.0);
        let mut account = Account::empty(client);

        let tx_block_store = &TxBlockStore::try_new().expect("Failed to initialize block store.");
        let tx_processor = TxProcessor::new(tx_block_store);
        let result =
            tx_processor.handle_withdrawal(&mut account, Withdrawal::new(client, tx, amount));

        assert!(matches!(
            result,
            Err(Error::WithdrawalAmountNegative { client, tx, amount })
            if client == ClientId::new(1) && tx == TxId::new(2) && amount == dec!(-1.0)
        ));
        Ok(())
    }
}
