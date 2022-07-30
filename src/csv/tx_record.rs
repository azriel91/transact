use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{
    model::{Chargeback, ClientId, Deposit, Dispute, Resolve, Transaction, TxId, Withdrawal},
    Error,
};

/// Represents a transaction record.
///
/// In contrast to the types in `crate::model`, this is specifically
#[derive(Debug, Deserialize, Serialize)]
pub struct TxRecord {
    r#type: TxType,
    client: ClientId,
    tx: TxId,
    amount: Option<Decimal>,
}

/// Types of transactions.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TxType {
    /// Credit to the client's asset account.
    Deposit,
    /// Debit to the client's asset account.
    Withdrawal,
    /// Client's claim that a transaction was erroneous and should be reversed.
    Dispute,
    /// Resolution to a dispute, releasing the associated held funds.
    Resolve,
    /// Final state of a dispute and represents the client reversing a
    /// transaction.
    Chargeback,
}

impl TryFrom<TxRecord> for Transaction {
    type Error = Error;

    fn try_from(tx_record: TxRecord) -> Result<Transaction, Error> {
        let TxRecord {
            r#type,
            client,
            tx,
            amount,
        } = tx_record;
        let transaction = match r#type {
            TxType::Deposit => {
                let amount = amount.ok_or(Error::DepositAmountNotProvided { client, tx })?;
                Transaction::from(Deposit::new(client, tx, amount))
            }
            TxType::Withdrawal => {
                let amount = amount.ok_or(Error::WithdrawalAmountNotProvided { client, tx })?;
                Transaction::from(Withdrawal::new(client, tx, amount))
            }
            TxType::Dispute => Transaction::from(Dispute::new(client, tx)),
            TxType::Resolve => Transaction::from(Resolve::new(client, tx)),
            TxType::Chargeback => Transaction::from(Chargeback::new(client, tx)),
        };

        Ok(transaction)
    }
}

impl From<Transaction> for TxRecord {
    fn from(transaction: Transaction) -> TxRecord {
        match transaction {
            Transaction::Deposit(deposit) => Self::from(deposit),
            Transaction::Withdrawal(withdrawal) => Self::from(withdrawal),
            Transaction::Dispute(dispute) => Self::from(dispute),
            Transaction::Resolve(resolve) => Self::from(resolve),
            Transaction::Chargeback(chargeback) => Self::from(chargeback),
        }
    }
}

impl From<Deposit> for TxRecord {
    fn from(deposit: Deposit) -> Self {
        TxRecord {
            r#type: TxType::Deposit,
            client: deposit.client(),
            tx: deposit.tx(),
            amount: Some(deposit.amount()),
        }
    }
}

impl From<Withdrawal> for TxRecord {
    fn from(withdrawal: Withdrawal) -> Self {
        TxRecord {
            r#type: TxType::Withdrawal,
            client: withdrawal.client(),
            tx: withdrawal.tx(),
            amount: Some(withdrawal.amount()),
        }
    }
}

impl From<Dispute> for TxRecord {
    fn from(dispute: Dispute) -> Self {
        TxRecord {
            r#type: TxType::Dispute,
            client: dispute.client(),
            tx: dispute.tx(),
            amount: None,
        }
    }
}

impl From<Resolve> for TxRecord {
    fn from(resolve: Resolve) -> Self {
        TxRecord {
            r#type: TxType::Resolve,
            client: resolve.client(),
            tx: resolve.tx(),
            amount: None,
        }
    }
}

impl From<Chargeback> for TxRecord {
    fn from(chargeback: Chargeback) -> Self {
        TxRecord {
            r#type: TxType::Chargeback,
            client: chargeback.client(),
            tx: chargeback.tx(),
            amount: None,
        }
    }
}
