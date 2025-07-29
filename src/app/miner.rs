use crate::blockchain::blockchain::Blockchain;
use crate::wallet::transaction_pool::TransactionPool;
use crate::wallet::transaction::RewardTransaction;
use crate::wallet::wallet::Wallet;
use crate::app::p2p_server::P2pServer;


pub struct Miner<'a>{
    pub blockchain:&'a Blockchain,
    pub transaction_pool: &'a TransactionPool,
    pub wallet:&'a Wallet,
    pub p2p_server:&'a P2pServer,
}

impl<'a>Miner<'a>{
    pub fn new(
        blockchain:&'a Blockchain,
        transaction_pool: &'a TransactionPool,
        wallet:&'a Wallet,
        p2p_server:&'a P2pServer,
    ) -> Self {
        Self {
            blockchain,
            transaction_pool,
            wallet,
            p2p_server,
        }
    }

    pub fn mine(&self){
        let next_block_number = self.blockchain.latest_block.block_number + 1;
        let valid_transactions = self.transaction_pool.valid_transactions();
        //include a reward for the miner
        let reward_tx = RewardTransaction::new(self.wallet.public_key.clone(),next_block_number);
        valid_transactions.push(reward_tx);
        //create a block consisting of the valid transactions
        self.blockchain.add_block(valid_transactions);
        //synchronize the chains in the peer-to-peer server
        self.p2p_server.sync_chains();
        //clear the transaction pool
        self.transaction_pool.clear();
        //broadcast to every miner to clear their transaction pool 
        println!("Mining Block with {} valid transctions.",valid_transactions.len());
    }


}