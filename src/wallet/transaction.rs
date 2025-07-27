use k256::ecdsa::{VerifyingKey,Signature};
use uuid::Uuid;
use std::collections::HashMap;
use crate::chain_util::ChainUtil;
use crate::wallet::wallet::Wallet;
use serde::{Serialize,Deserialize}; 

#[derive(Serialize,Deserialize)]
pub struct Input {
    pub timestamp:u128,
    pub amount:u64,
    pub address:String,
    pub signature:Vec<u8>,
}

pub struct Transaction {
    pub id:String,
    pub input: Option<Input>,
    pub outputs: Vec<Output>,
}

#[derive(Clone,Serialize,Deserialize)]
pub struct Output {
    pub amount: u64,
    pub address: String,
}


impl Transaction{
    pub fn new_transaction(sender_wallet: &Wallet,recipient:String,amount:u64)->Option<Self>{
        if amount > sender_wallet.balance {
            eprintln!("Amount: {} exceeds balance.",amount);
            return None;
        }
        let mut outputs = Vec::new();

        outputs.push(Output{
            amount:sender_wallet.balance - amount,
            address: sender_wallet.public_key.clone(),
        });

        outputs.push(Output{
            amount,
            address:recipient,
        });

        let mut transaction = Transaction{
            id: ChainUtil::id(),
            input:None,
            outputs,
        };

        transaction.sign_transaction(sender_wallet);

        Some(transaction)
    }

    fn sign_transaction(&mut self , sender_wallet:&Wallet){
        let hash = ChainUtil::hash(&self.outputs);
        let signature = sender_wallet.sign(&hash);
        self.input = Some(Input{
            timestamp: chrono::Utc::now().timestamp_millis() as u128,
            amount:sender_wallet.balance,
            address: sender_wallet.public_key.clone(),
            signature: signature.as_ref().to_vec(),
        })
    }

    pub fn verify_transaction(transaction:&Transaction)->bool{
        if let Some(input)=&transaction.input{
            ChainUtil::verify_signature(
                &input.address,
                &input.signature,
                &ChainUtil::hash(&transaction.outputs),
            )
        } else {
            println!("false called in verify sign");
            false
        }
    }
}
