use crate::wallet::wallet::Wallet;
use crate::wallet::transaction::{Transaction};
use crate::wallet::transaction_pool::TransactionPool;


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn adds_a_transaction_to_the_pool(){
        let mut tp = TransactionPool::new();
        let wallet = Wallet::new();
        let transaction = Transaction::new_transaction(&wallet,"random-address".to_string(),30)
            .expect("Transaction creation failed");
        
        tp.update_or_add_transaction(transaction.clone());

        let found = tp.transactions.iter().find(|t| t.id==transaction.id);
        assert!(found.is_some(),"transaction should be found in the pool");
        assert_eq!(&transaction,found.unwrap(),"found transaction should be same a original");
    }

    #[test]
    fn updates_a_transaction_in_the_pool() {
        let mut tp = TransactionPool::new();
        let wallet = Wallet::new();
        let mut transaction = Transaction::new_transaction(&wallet, "random-address".to_string(), 30)
            .expect("Transaction creation should succeed");

        tp.update_or_add_transaction(transaction.clone());
        let old_serialized = serde_json::to_string(&transaction).expect("Serialize old transaction");
        transaction.update(&wallet, "2ndaddress".to_string(), 40).expect("Update should succeed");
        tp.update_or_add_transaction(transaction.clone());

        let new_serialized = serde_json::to_string(
            tp.transactions
                .iter()
                .find(|t| t.id == transaction.id)
                .expect("Updated transaction must be found in pool")
        ).expect("Serialize new transaction");

        assert_ne!(new_serialized, old_serialized, "Updated transaction should not equal old transaction");
    }
}