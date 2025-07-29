use crate::blockchain::blockchain::Blockchain;
use crate::wallet::transaction_pool::TransactionPool;
use crate::wallet::transaction::RewardTransaction;
use crate::wallet::wallet::Wallet;
use crate::app::p2p_server::P2pServer;
use crate::wallet::transaction::ChainTransaction;
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;

pub struct Miner{
    pub blockchain:Arc<TokioMutex<Blockchain>>,
    pub transaction_pool: Arc<TokioMutex<TransactionPool>>,
    pub wallet:Arc<Wallet>,
    pub p2p_server:Arc<P2pServer>,
}

impl Miner{
    pub fn new(
        blockchain:Arc<TokioMutex<Blockchain>>,
        transaction_pool:Arc<TokioMutex<TransactionPool>>,
        wallet: Arc<Wallet>,
        p2p_server: Arc<P2pServer>,
    ) -> Self {
        Self {
            blockchain,
            transaction_pool,
            wallet,
            p2p_server,
        }
    }

    pub async fn mine(&self){
        println!("mine called");
        let next_block_number = {
            let chain = self.blockchain.lock().await;
            chain.latest_block().block_number + 1
        };
        let transactions: Vec<ChainTransaction> = {
            let pool = self.transaction_pool.lock().await;
            pool.valid_transactions(self.blockchain.clone()).await
                .iter()
                .map(|t| ChainTransaction::Normal((*t).clone()))
                .collect()
        };
        
        //include a reward for the miner
        let mut chain_transactions = transactions;
        chain_transactions.push(ChainTransaction::Reward(
            RewardTransaction::new(self.wallet.public_key.clone(),next_block_number)
        ));
        println!("chain txns: {:?}",chain_transactions);
        //create a block consisting of the valid transactions
        {
            let mut chain = self.blockchain.lock().await;
            chain.add_block(chain_transactions.clone());
        }
         println!("add block finished ");
        //synchronize the chains in the peer-to-peer server
        self.p2p_server.sync_chains().await;
        //clear the transaction pool
        {
            let mut pool = self.transaction_pool.lock().await;
            pool.clear();
        }
        //broadcast to every miner to clear their transaction pool 
        self.p2p_server.broadcast_clear_transactions().await;
        println!("Mining Block with {} valid transctions.",chain_transactions.len());
    }


}