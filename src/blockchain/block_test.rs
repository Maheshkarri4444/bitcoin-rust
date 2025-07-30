// use super::block::Block;
// use crate::config::DIFFICULTY;

// #[test]
// fn it_sets_data_to_match_input(){
//     let data = vec!["example".to_string()];
//     let last_block = Block::genesis();
//     let block = Block::mine_block(&last_block,data.clone());

//     assert_eq!(block.data,data);
// }

// #[test]
// fn it_sets_last_hash_to_match_last_block_hash(){
//     let data = vec!["bar".to_string()];
//     let last_block = Block::genesis();
//     let block = Block::mine_block(&last_block, data);

//     assert_eq!(block.last_hash, last_block.hash);
// }

// #[test]
// fn it_generates_a_hash_that_matches_difficulty(){
//     let data = vec!["testdata".to_string()];
//     let last_block = Block::genesis();
//     let block = Block::mine_block(&last_block,data);

//     let prefix = &block.hash[..block.difficulty];
//     assert!(prefix.chars().all(|c| c=='0'),"Hash does not match difficulty:{}",block.hash);
// }

// #[test]
// fn it_lowers_the_difficulty_for_slowly_mined_blocks(){
//     let last_block = Block::genesis();
//     let new_timestamp = last_block.timestamp + 360_000;

//     let decreased_difficulty = Block::adjust_difficulty(&last_block,new_timestamp);
//     assert_eq!(decreased_difficulty,last_block.difficulty -1);
// }

// #[test]
// fn it_raises_the_difficulty_for_quickly_mined_blocks(){
//     let last_block = Block::genesis();
//     let new_timestamp = last_block.timestamp + 100;
//     let increased_difficulty = Block::adjust_difficulty(&last_block,new_timestamp);
//     assert_eq!(increased_difficulty,last_block.difficulty +1);

// }