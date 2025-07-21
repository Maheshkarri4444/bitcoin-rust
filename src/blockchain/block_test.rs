use super::block::Block;

#[test]
fn it_sets_data_to_match_input(){
    let data="example".to_string();
    let last_block = Block::genesis();
    let block = Block::mine_block(&last_block,data.clone());

    assert_eq!(block.data,data);
}

#[test]
fn it_sets_last_hash_to_match_last_block_hash(){
    let data = "bar".to_string();
    let last_block = Block::genesis();
    let block = Block::mine_block(&last_block, data);

    assert_eq!(block.last_hash, last_block.hash);
}