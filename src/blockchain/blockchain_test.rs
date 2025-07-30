// use super::blockchain::Blockchain;
// use super::block::Block;

// #[test]
// fn test_genesis_block(){
//     let bc = Blockchain::new();
//     assert_eq!(bc.chain[0],Block::genesis());
// }

// #[test]
// fn test_add_block(){
//     let mut bc = Blockchain::new();
//     let data = vec!["foo".to_string()];
//     bc.add_block(data.clone());

//     let last_block = bc.chain.last().unwrap();
//     assert_eq!(last_block.data,data);
//     assert_eq!(last_block.block_number,1);
//     assert_eq!(last_block.last_hash,Block::genesis().hash);
// }

// #[test]
// fn test_valid_chain(){
//     let mut bc1 = Blockchain::new();
//     let mut bc2 = Blockchain::new();

//     bc2.add_block(vec!["foo".to_string()]);

//     assert!(Blockchain::is_valid_chain(&bc2.chain));
// }

// #[test]
// fn test_invalid_genesis_block() {
//     let mut bc = Blockchain::new();
//     let mut bc2 = Blockchain::new();

//     bc2.chain[0].data = vec!["Bad data".to_string()];
//     assert!(!Blockchain::is_valid_chain(&bc2.chain));
// }


// #[test]
// fn test_invalid_chain_data() {
//     let mut bc = Blockchain::new();
//     let mut bc2 = Blockchain::new();

//     bc2.add_block(vec!["foo".to_string()]);
//     bc2.chain[1].data = vec!["Not foo".to_string()]; // Corrupt data

//     assert!(!Blockchain::is_valid_chain(&bc2.chain));
// }

// #[test]
// fn test_replace_chain_valid() {
//     let mut bc = Blockchain::new();
//     let mut bc2 = Blockchain::new();

//     bc2.add_block(vec!["goo".to_string()]);

//     bc.replace_chain(bc2.chain.clone());
//     assert_eq!(bc.chain, bc2.chain);
// }


// #[test]
// fn test_replace_chain_invalid_or_shorter(){
//     let mut bc = Blockchain::new();
//     let mut bc2 = Blockchain::new();

//     bc.add_block(vec!["foo".to_string()]);
//     bc.replace_chain(bc2.chain.clone());

//     assert_ne!(bc.chain, bc2.chain);
// }