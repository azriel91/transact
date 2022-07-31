use std::cmp::Ordering;

use crate::{
    model::{Account, Chargeback, Deposit, Dispute, Resolve, Transaction, Withdrawal},
    Error,
};

/// Processes transactions for an account.
#[derive(Debug)]
pub struct TxProcessor;

impl TxProcessor {
    /// Processes a transaction for an account.
    pub fn process(account: &mut Account, transaction: Transaction) -> Result<(), Error> {
        match transaction {
            Transaction::Deposit(deposit) => Self::handle_deposit(account, deposit),
            Transaction::Withdrawal(withdrawal) => Self::handle_withdrawal(account, withdrawal),
            Transaction::Dispute(dispute) => Self::handle_dispute(account, dispute),
            Transaction::Resolve(resolve) => Self::handle_resolve(account, resolve),
            Transaction::Chargeback(chargeback) => Self::handle_chargeback(account, chargeback),
        }
    }

    fn handle_deposit(account: &mut Account, deposit: Deposit) -> Result<(), Error> {
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
                Account::try_new(client, available_next, account.held(), account.locked())
                    .map_err(|_| Error::DepositTotalOverflow { client, tx })
            })
            .map(|account_updated| *account = account_updated)
    }

    fn handle_withdrawal(account: &mut Account, withdrawal: Withdrawal) -> Result<(), Error> {
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
            // Not enough funds, don't change the account values.
            Ok(())
        } else {
            let available_next = available.saturating_sub(withdrawal.amount());
            let account_updated = Account::try_new(
                client,
                available_next,
                account.held(),
                account.locked(),
            )
            .expect(
                "Overflow impossible: Withdrawal amount is less than or equal to available amount, \
                        and is non-negative.",
            );
            *account = account_updated;

            Ok(())
        }
    }

    fn handle_dispute(_account: &mut Account, _dispute: Dispute) -> Result<(), Error> {
        todo!()
    }

    fn handle_resolve(_account: &mut Account, _resolve: Resolve) -> Result<(), Error> {
        todo!()
    }

    fn handle_chargeback(_account: &mut Account, _chargeback: Chargeback) -> Result<(), Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;

    use super::TxProcessor;
    use crate::{
        model::{Account, ClientId, Deposit, TxId},
        Error,
    };

    #[test]
    fn deposit_positive_amount() -> Result<(), Error> {
        let client = ClientId::new(1);
        let tx = TxId::new(2);
        let amount = dec!(1.0);
        let mut account = Account::empty(client);

        TxProcessor::handle_deposit(&mut account, Deposit::new(client, tx, amount))?;

        let account_expected =
            Account::try_new(client, dec!(1.0), dec!(0.0), false).expect("Test data invalid.");
        assert_eq!(account_expected, account);
        Ok(())
    }

    #[test]
    fn deposit_negative_amount_returns_err() -> Result<(), Error> {
        let client = ClientId::new(1);
        let tx = TxId::new(2);
        let amount = dec!(-1.0);
        let mut account = Account::empty(client);

        let result = TxProcessor::handle_deposit(&mut account, Deposit::new(client, tx, amount));

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
        let mut account =
            Account::try_new(client, dec!(1.0), dec!(0.0), false).expect("Test data invalid.");

        let result = TxProcessor::handle_deposit(&mut account, Deposit::new(client, tx, amount));

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
        let mut account =
            Account::try_new(client, dec!(1.0), dec!(2.0), false).expect("Test data invalid.");

        let result = TxProcessor::handle_deposit(&mut account, Deposit::new(client, tx, amount));

        assert!(matches!(
            result,
            Err(Error::DepositTotalOverflow { client, tx })
            if client == ClientId::new(1) && tx == TxId::new(2)
        ));
        Ok(())
    }
}
