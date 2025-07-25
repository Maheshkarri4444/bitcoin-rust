use std::fmt;

pub struct Wallet {
    pub balance: u64,
    pub key_pair: Option<String>,
    pub public_key: Option<String>, 
}

impl Wallet {
    pub fn new() -> Self{
        Self {
            balance:crate::config::INITIAL_BALANCE,
            key_pair: None,
            public_key: None,
        }
    }
}

impl fmt::Display for Wallet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>)-> fmt::Result {
        write!{
            f,
            "Wallet -\n publicKey: {}\n balance:{}",
            self.public_key.as_deref().unwrap_or("None"),
            self.balance
        }
    }
}