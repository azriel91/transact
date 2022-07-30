use crate::model::{Account, Chargeback, Deposit, Dispute, Resolve, Transaction, Withdrawal};

/// Processes transactions for an account.
#[derive(Debug)]
pub struct TxProcessor;

impl TxProcessor {
    /// Processes a transaction for an account.
    pub fn process(account: &mut Account, transaction: Transaction) {
        match transaction {
            Transaction::Deposit(deposit) => Self::handle_deposit(account, deposit),
            Transaction::Withdrawal(withdrawal) => Self::handle_withdrawal(account, withdrawal),
            Transaction::Dispute(dispute) => Self::handle_dispute(account, dispute),
            Transaction::Resolve(resolve) => Self::handle_resolve(account, resolve),
            Transaction::Chargeback(chargeback) => Self::handle_chargeback(account, chargeback),
        }
    }

    fn handle_deposit(_account: &mut Account, _deposit: Deposit) {
        todo!()
    }

    fn handle_withdrawal(_account: &mut Account, _withdrawal: Withdrawal) {
        todo!()
    }

    fn handle_dispute(_account: &mut Account, _dispute: Dispute) {
        todo!()
    }

    fn handle_resolve(_account: &mut Account, _resolve: Resolve) {
        todo!()
    }

    fn handle_chargeback(_account: &mut Account, _chargeback: Chargeback) {
        todo!()
    }
}
