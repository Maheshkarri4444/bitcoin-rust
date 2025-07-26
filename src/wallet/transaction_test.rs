use crate::wallet::wallet::Wallet;
use crate::wallet::transaction::{Transaction,Output};


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn outputs_amount_subtracted_from_wallet_balance(){
        let wallet = Wallet::new();
        let recipient = String::from("recipient public address");
        let amount = 50;

        let transaction = Transaction::new_transaction(&wallet,recipient.clone(),amount)
            .expect("Transaction should be created");
        
        let sender_output = transaction.outputs.iter()
            .find(|output| output.address == wallet.public_key);
        assert!(sender_output.is_some(),"Sender output not found");
        assert_eq!(sender_output.unwrap().amount,wallet.balance - amount);
    }

    #[test]
        fn outputs_amount_added_to_recipient() {
        let wallet = Wallet::new();
        let recipient = String::from("recipient_public_key");
        let amount = 50;

        let transaction = Transaction::new_transaction(&wallet, recipient.clone(), amount)
            .expect("Transaction should be created");

        let recipient_output = transaction.outputs.iter()
            .find(|output| output.address == recipient);

        assert!(recipient_output.is_some(), "Recipient output not found");
        assert_eq!(recipient_output.unwrap().amount, amount);
    }

    #[test]
        fn transacting_with_amount_exceeding_balance_returns_none() {
        let wallet = Wallet::new();
        let recipient = String::from("recipient_public_key");
        let amount = wallet.balance + 1000;  // definitely exceed balance

        let transaction = Transaction::new_transaction(&wallet, recipient, amount);

        assert!(transaction.is_none(), "Transaction should not be created when amount exceeds balance");
    }

    #[test]
    fn test_transaction_outputs_subtract_and_add_correctly() {
        let wallet = Wallet::new();
        let recipient = "recipientpublickey".to_string();
        let amount = 50;
        let transaction = Transaction::new_transaction(&wallet, recipient.clone(), amount).unwrap();

        // Output for wallet: balance - amount
        let sender_output = transaction
            .outputs
            .iter()
            .find(|o| o.address == wallet.public_key)
            .expect("Sender output not found");
        assert_eq!(sender_output.amount, wallet.balance - amount);

        let recipient_output = transaction
            .outputs
            .iter()
            .find(|o| o.address == recipient)
            .expect("Recipient output not found");
        assert_eq!(recipient_output.amount, amount);

        assert_eq!(transaction.input.as_ref().unwrap().amount, wallet.balance);
    }
}

