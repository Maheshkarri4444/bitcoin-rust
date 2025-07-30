// use crate::wallet::wallet::Wallet;
// use crate::wallet::transaction_pool::TransactionPool;

// #[cfg(test)]
// mod tests{
//     use super::*;

//     #[test]
//     fn creating_a_transaction_and_repeating_it_updates_correctly(){
//         let wallet = Wallet::new();
//         let mut tp = TransactionPool::new();

//         let send_amount = 50u64;
//         let recipient = String::from("r4nd-4ddr355");

//         wallet.create_transaction(recipient.clone(),send_amount,&mut tp)
//             .expect("Transaction creation should succeed");

//         wallet.create_transaction(recipient.clone(),send_amount,&mut tp);

//         let transaction = tp.existing_transaction(&wallet.public_key)
//             .expect("Transaction should exist in pool")
//             .clone();

//         let sender_output = transaction.outputs.iter()
//             .find(|output| output.address == wallet.public_key)
//             .expect("Sender output should exist after transaction");
        
//         assert_eq!(
//             sender_output.amount,
//             wallet.balance -send_amount * 2,
//             "Sender output should subtracted double the send amount"
//         );

//         let recipient_amounts: Vec<u64>= transaction.outputs.iter()
//             .filter(|output| output.address == recipient)
//             .map(|output| output.amount)
//             .collect();

        
//         assert_eq!(recipient_amounts,vec![send_amount,send_amount],"Recipient should have 2 outputs with send_amount each")

//     }
// }
