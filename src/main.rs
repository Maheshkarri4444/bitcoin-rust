mod blockchain;

use blockchain::block::Block;

fn main(){
    let genesis = Block::genesis();
    println!("{}",genesis);

    let new_block = Block::mine_block(&genesis,"some-data".to_string());
    println!("{}",new_block);
}