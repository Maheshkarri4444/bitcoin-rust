use super::block::Block;

#[derive(Debug)]
pub struct Blockchain{
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new()->Self{
        Blockchain {
            chain:vec![Block::genesis()],
        }
    }

    pub fn add_block(&mut self,data:String)->Block{
        let last_block=self.chain.last().expect("Blockchain should have atleat one block");
        let new_block= Block::mine_block(last_block,data);
        self.chain.push(new_block.clone());
        self.chain.last().unwrap().clone()
    }

    pub fn is_valid_chain(chain: &[Block])->bool{
        if chain.is_empty()||chain[0]!=Block::genesis(){
            return false;
        }
        for i in 1..chain.len(){
            let block = &chain[i];
            let last_block = &chain[i-1];

            if block.last_hash !=last_block.hash{
                return false;
            }

            if block.hash != Block::hash(block.block_number,block.timestamp,&block.last_hash,&block.data,&block.nonce,&block.difficulty){
                return false;
            }
        }
        true
    }

    pub fn replace_chain(&mut self,new_chain:Vec<Block>){
        if new_chain.len() <= self.chain.len(){
            println!("Recieved chain is no longer than the current chain");
            return;
        } else if !Self::is_valid_chain(&new_chain){
            println!("The recieved chain is not valid");
            return;
        }

        println!("Replacing Blockchain with the new chain");
        self.chain = new_chain;

    }


}
