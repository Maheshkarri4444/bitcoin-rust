// use crate::wallet::wallet::Wallet;
// use crate::wallet::transaction::{Transaction,Output,RewardTransaction};


// #[cfg(test)]
// mod tests{
//     use super::*;

//     #[test]
//     fn outputs_amount_subtracted_from_wallet_balance(){
//         let wallet = Wallet::new();
//         let recipient = String::from("recipient public address");
//         let amount = 50;

//         let transaction = Transaction::new_transaction(&wallet,recipient.clone(),amount)
//             .expect("Transaction should be created");
        
//         let sender_output = transaction.outputs.iter()
//             .find(|output| output.address == wallet.public_key);
//         assert!(sender_output.is_some(),"Sender output not found");
//         assert_eq!(sender_output.unwrap().amount,wallet.balance - amount);
//     }

//     #[test]
//         fn outputs_amount_added_to_recipient() {
//         let wallet = Wallet::new();
//         let recipient = String::from("recipient_public_key");
//         let amount = 50;

//         let transaction = Transaction::new_transaction(&wallet, recipient.clone(), amount)
//             .expect("Transaction should be created");

//         let recipient_output = transaction.outputs.iter()
//             .find(|output| output.address == recipient);

//         assert!(recipient_output.is_some(), "Recipient output not found");
//         assert_eq!(recipient_output.unwrap().amount, amount);
//     }

//     #[test]
//         fn transacting_with_amount_exceeding_balance_returns_none() {
//         let wallet = Wallet::new();
//         let recipient = String::from("recipient_public_key");
//         let amount = wallet.balance + 1000;  // definitely exceed balance

//         let transaction = Transaction::new_transaction(&wallet, recipient, amount);

//         assert!(transaction.is_none(), "Transaction should not be created when amount exceeds balance");
//     }

//     #[test]
//     fn test_transaction_outputs_subtract_and_add_correctly() {
//         let wallet = Wallet::new();
//         let recipient = "recipientpublickey".to_string();
//         let amount = 50;
//         let transaction = Transaction::new_transaction(&wallet, recipient.clone(), amount).unwrap();

//         // Output for wallet: balance - amount
//         let sender_output = transaction
//             .outputs
//             .iter()
//             .find(|o| o.address == wallet.public_key)
//             .expect("Sender output not found");
//         assert_eq!(sender_output.amount, wallet.balance - amount);

//         let recipient_output = transaction
//             .outputs
//             .iter()
//             .find(|o| o.address == recipient)
//             .expect("Recipient output not found");
//         assert_eq!(recipient_output.amount, amount);

//         assert_eq!(transaction.input.as_ref().unwrap().amount, wallet.balance);
//     }


//     // ----- New test for validating a valid transaction -----
//     #[test]
//     fn validates_a_valid_transaction() {
//         let wallet = Wallet::new();
//         let recipient = String::from("recipient_public_key");
//         let amount = 50;

//         let transaction = Transaction::new_transaction(&wallet, recipient, amount)
//             .expect("Transaction should be created");

//         assert!(Transaction::verify_transaction(&transaction), "Transaction signature should be valid");
//     }
//     // ----- New test for invalidating a corrupt transaction -----
//     #[test]
//     fn invalidates_a_corrupt_transaction() {
//         let wallet = Wallet::new();
//         let recipient = String::from("recipient_public_key");
//         let amount = 50;

//         let mut transaction = Transaction::new_transaction(&wallet, recipient.clone(), amount)
//             .expect("Transaction should be created");
        
//         // Corrupt the transaction by tampering with the first output's amount
//         if let Some(first_output) = transaction.outputs.get_mut(0) {
//             first_output.amount = 50000;
//         }

//         // Expect the verification to fail
//         assert!(!Transaction::verify_transaction(&transaction), "Corrupt transaction should fail verification");
//     }

//     #[test]
//     fn updating_a_transaction_subtracts_and_outputs_to_next_recipient(){
//         let wallet = Wallet::new();
//         let recipient = String::from("recipient_public_key");
//         let amount = 50;
//         let mut transaction = Transaction::new_transaction(&wallet, recipient.clone(), amount)
//             .expect("Transaction should be created");

//         let next_amount = 20;
//         let next_recipient = "next-address".to_string();

//         let update_result = transaction.update(&wallet, next_recipient.clone(), next_amount);
//         assert!(update_result.is_some(), "Transaction update should succeed");

//         let sender_output = transaction.outputs.iter()
//             .find(|output| output.address == wallet.public_key)
//             .expect("Sender output should exist after update");
//         assert_eq!(
//             sender_output.amount,
//             wallet.balance - amount - next_amount,
//             "Sender output should subtract both amounts"
//         );

//         let next_recipient_output = transaction.outputs.iter()
//             .find(|output| output.address == next_recipient)
//             .expect("Next recipient output should exist after update");
//         assert_eq!(
//             next_recipient_output.amount,
//             next_amount,
//             "Next recipient output should match the updated amount"
//         );
//     }

//     #[test]
//     fn creates_valid_reward_transaction() {
//         let miner_address = String::from("miner_public_key_123");
//         let block_height = 100;

//         let reward_tx = RewardTransaction::new(miner_address.clone(), block_height);

//         assert_eq!(reward_tx.output.amount, crate::config::MINING_REWARD);

//         assert_eq!(reward_tx.output.address, miner_address);

//         assert!(
//             reward_tx.coinbase.contains(&block_height.to_string()),
//             "Coinbase field should include block height"
//         );

//         assert!(!reward_tx.id.is_empty(), "RewardTransaction id should not be empty");
//     }
// }




