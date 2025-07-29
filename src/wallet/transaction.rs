use k256::ecdsa::{VerifyingKey,Signature};
use uuid::Uuid;
use std::collections::HashMap;
use crate::chain_util::ChainUtil;
use crate::wallet::wallet::Wallet;
use serde::{Serialize,Deserialize}; 
use crate::config::MINING_REWARD;

#[derive(Clone,Serialize,Deserialize,Debug,PartialEq)]
pub struct Input {
    pub timestamp:u128,
    pub amount:u64,
    pub address:String,
    pub signature:Vec<u8>,
}
#[derive(Serialize,Deserialize,Debug,PartialEq,Clone)]
pub struct Transaction {
    pub id:String,
    pub input: Option<Input>,
    pub outputs: Vec<Output>,
}

#[derive(Clone,Serialize,Deserialize,PartialEq,Debug)]
pub struct Output {
    pub amount: u64,
    pub address: String,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct RewardTransaction {
    pub id: String,
    pub coinbase: String,
    pub output: Output,
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

    pub fn update(&mut self , sender_wallet:&Wallet,recipient:String,amount:u64)->Option<()>{
        let sender_output_opt = self.outputs.iter_mut().find(|output| output.address == sender_wallet.public_key);
        let sender_output = match sender_output_opt{
            Some(o)=>o,
            None=>{
                println!("Sender not found in transaction outputs");
                return None;
            }
        };
        if amount >sender_output.amount{
            println!("Amount: {} exceeds the balance",amount);
            return None;
        }

        sender_output.amount -= amount;
        self.outputs.push(Output{
            amount,
            address:recipient,
        });
        self.sign_transaction(sender_wallet);
        Some(())
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

impl RewardTransaction {
    pub fn new(miner_address: String, block_height: u64) -> Self {
    let coinbase = format!(
        "{}-{}", 
        block_height,
        Uuid::new_v4() // unique extranonce
    );

    Self {
        id: ChainUtil::id(),
        coinbase,
        output: Output {
            amount: MINING_REWARD,
            address: miner_address,
        },
    }
    }
}

