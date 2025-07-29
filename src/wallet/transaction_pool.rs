use crate::wallet::transaction::Transaction;
use serde::{Serialize,Deserialize};
use crate::blockchain::blockchain::Blockchain;
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;

#[derive(Serialize,Deserialize)]
pub struct TransactionPool{
    pub transactions:Vec<Transaction>,
}

impl TransactionPool {
    pub fn new() -> Self{
        Self{
            transactions:Vec::new(),
        }
    }

    pub fn update_or_add_transaction(&mut self,transaction:Transaction){
        if let Some(pos)=self.transactions.iter().position(|t| t.id == transaction.id){
            self.transactions[pos] = transaction;
        } else {
            self.transactions.push(transaction);
        }
    }

    pub fn existing_transaction(&self, address:&str)-> Option<&Transaction>{
        self.transactions.iter().find(|t|{
            if let Some(input) = &t.input{
                input.address == address
            } else {
                false
            }
        })
    }

    pub async fn valid_transactions(
        &self,
        blockchain: Arc<TokioMutex<Blockchain>>,
    ) -> Vec<&Transaction> {
        let mut valid_txs = Vec::new();

        for transaction in &self.transactions {
            let output_total: u64 = transaction.outputs.iter().map(|output| output.amount).sum();

            if let Some(input) = &transaction.input {
                if input.amount != output_total {
                    println!("Invalid transaction from: {}", input.address);
                    continue;
                }

                // Await async verify_transaction
                if !Transaction::verify_transaction(transaction, &blockchain).await {
                    println!("Invalid signature from: {}", input.address);
                    continue;
                }

                valid_txs.push(transaction);
            } else {
                println!("Transaction missing input");
                // You can choose whether to include or exclude this transaction
            }
        }

        valid_txs
    }

    pub fn clear(&mut self){
        self.transactions.clear();
    }


}
