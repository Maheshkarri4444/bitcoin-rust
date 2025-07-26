use bitcoin_rust::blockchain::blockchain::Blockchain;
use bitcoin_rust::chain_util;
use bitcoin_rust::wallet::wallet::Wallet;

fn main() {
    let mut bc = Blockchain::new();

    for i in 0..10 {
        let block = bc.add_block(vec![format!("block {}", i)]);
        println!("{}", block);
    }

    let wallet = Wallet::new();
    println!("{}", wallet);
}
