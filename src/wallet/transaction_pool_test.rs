// use crate::wallet::wallet::Wallet;
// use crate::wallet::transaction::{Transaction};
// use crate::wallet::transaction_pool::TransactionPool;


// #[cfg(test)]
// mod tests{
//     use super::*;

//     #[test]
//     fn adds_a_transaction_to_the_pool(){
//         let mut tp = TransactionPool::new();
//         let wallet = Wallet::new();
//         let transaction = Transaction::new_transaction(&wallet,"random-address".to_string(),30)
//             .expect("Transaction creation failed");
        
//         tp.update_or_add_transaction(transaction.clone());

//         let found = tp.transactions.iter().find(|t| t.id==transaction.id);
//         assert!(found.is_some(),"transaction should be found in the pool");
//         assert_eq!(&transaction,found.unwrap(),"found transaction should be same a original");
//     }

//     #[test]
//     fn updates_a_transaction_in_the_pool() {
//         let mut tp = TransactionPool::new();
//         let wallet = Wallet::new();
//         let mut transaction = Transaction::new_transaction(&wallet, "random-address".to_string(), 30)
//             .expect("Transaction creation should succeed");

//         tp.update_or_add_transaction(transaction.clone());
//         let old_serialized = serde_json::to_string(&transaction).expect("Serialize old transaction");
//         transaction.update(&wallet, "2ndaddress".to_string(), 40).expect("Update should succeed");
//         tp.update_or_add_transaction(transaction.clone());

//         let new_serialized = serde_json::to_string(
//             tp.transactions
//                 .iter()
//                 .find(|t| t.id == transaction.id)
//                 .expect("Updated transaction must be found in pool")
//         ).expect("Serialize new transaction");

//         assert_ne!(new_serialized, old_serialized, "Updated transaction should not equal old transaction");
//     }

//     #[test]
//     fn mixes_valid_and_corrupt_transactions(){
//         let mut tp = TransactionPool::new();
//         let mut wallet = Wallet::new();
//         let mut valid_transactions: Vec<Transaction> = Vec::new();


//         let transaction = wallet.create_transaction("random-address".to_string(),30,&mut tp)
//             .expect("Transaction creation failed");
//         valid_transactions.push(transaction.clone());

//         for i in 0..6{
//             wallet = Wallet::new();
//             let mut transaction =wallet.create_transaction("4rnd-4dre55".to_string(),30,&mut tp)
//                 .expect("Transaction creation failed");
            
//             if i%2==0{
//                 if let Some(input) = transaction.input.as_mut(){
//                     input.amount = 99999;
//                 }
//                 tp.update_or_add_transaction(transaction);
//             } else {
//                 tp.update_or_add_transaction(transaction.clone());
//                 valid_transactions.push(transaction);
//             }
//         }
//         let pool_json = serde_json::to_string(&tp.transactions).expect("Serialize pool transactions");
//         let valid_json = serde_json::to_string(&valid_transactions).expect("Serialize valid transactions");
//         assert_ne!(pool_json,valid_json,"Pool transactions should not equal valid transactions");

//         let filtered_valid = tp.valid_transactions();
//         assert_eq!(filtered_valid.len(),valid_transactions.len());
//         for vt in valid_transactions.iter(){
//             assert!(filtered_valid.iter().any(|t| t.id ==vt.id),"Valid transaction missing from filtered list");
//         }
//     }
// }