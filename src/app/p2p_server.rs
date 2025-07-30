use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::TcpListener;
use tokio_tungstenite::{
    accept_async, connect_async,
    tungstenite::protocol::Message,
    WebSocketStream, MaybeTlsStream,
};
use futures::{StreamExt, SinkExt};
use tokio::net::TcpStream;
use url::Url;
use tokio::time::{timeout, Duration};
use serde_json::Value;
use crate::TokioMutex;

use crate::blockchain::block::Block;
use crate::blockchain::blockchain::Blockchain;

use crate::wallet::transaction_pool::TransactionPool;
use crate::wallet::transaction::Transaction;

//1st terminal cargo run --bin bitcoin_rust
//2nd terminal HTTP_PORT=3002 P2P_PORT=5002 PEERS=ws://127.0.0.1:5001 cargo run --bin bitcoin_rust
//3rd terminal HTTP_PORT=3003 P2P_PORT=5003 PEERS=ws://127.0.0.1:5001,ws://127.0.0.1:5002 cargo run --bin bitcoin_rust

const MESSAGE_TYPE_CHAIN: &str = "CHAIN";
const MESSAGE_TYPE_TRANSACTION: &str = "TRANSACTION";
const MESSAGE_TYPE_CLEAR_TRANSACTIONS: &str = "CLEAR_TRANSACTIONS";

/// A peer write-half and its reading task handle
#[derive(Clone)]
struct PeerSocket {
    write: Arc<Mutex<futures::stream::SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
}

pub struct P2pServer {
    pub blockchain: Arc<Mutex<Blockchain>>,
    pub transaction_pool: Arc<TokioMutex<TransactionPool>>,
    pub peers: Vec<String>,
    pub sockets: Arc<Mutex<Vec<PeerSocket>>>,
}

impl P2pServer {
    pub fn new(
        blockchain: Arc<Mutex<Blockchain>>, 
        transaction_pool: Arc<TokioMutex<TransactionPool>>,
        peers: Vec<String>,
        ) -> Self {
        Self {
            blockchain,
            transaction_pool,
            peers,
            sockets: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn listen(self: Arc<Self>, port: u16) {
        let addr = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(&addr).await.expect("Failed to bind");
        println!("Listening for peer to peer connections on: {}", addr);

        let self_clone = Arc::clone(&self);
        tokio::spawn(async move {
            self_clone.connect_to_peers().await;
        });

        while let Ok((stream, _)) = listener.accept().await {
            let stream = MaybeTlsStream::Plain(stream);
            let ws_stream = accept_async(stream)
                .await
                .expect("Failed to accept WebSocket connection");
            let self_clone = self.clone();
            tokio::spawn(async move {
                self_clone.connect_socket(ws_stream).await;
            });
        }
    }

    async fn connect_to_peers(self: Arc<Self>) {
        let peers = self.peers.clone();
        println!("peers in connect to peers: {:?}", peers);
        for peer in peers {
            println!("connect to peer: {}", peer);
            let result =
                timeout(Duration::from_secs(5), connect_async(Url::parse(&peer).unwrap())).await;
            match result {
                Ok(Ok((ws_stream, _))) => {
                    println!("Connected to peer: {}", peer);
                    let self_clone = Arc::clone(&self);
                    tokio::spawn(async move {
                        self_clone.connect_socket(ws_stream).await;
                    });
                }
                Ok(Err(e)) => {
                    eprintln!("Failed to connect to peer {}: {}", peer, e);
                }
                Err(_) => {
                    eprintln!("Timeout while connecting to peer {}", peer);
                }
            }
        }
    }

    async fn connect_socket(self: Arc<Self>, ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>) {
        println!("Peer socket connected!");

        // Split websocket stream into write and read halves
        let (write, mut read) = ws_stream.split();

        let write = Arc::new(Mutex::new(write));

        let self_clone = self.clone();
        tokio::spawn(async move {
            while let Some(msg_result) = read.next().await {
                match msg_result {
                    Ok(msg) if msg.is_text() => {
                        let text = msg.into_text().unwrap();
                        // println!("Received message: {}", text);
                        if let Ok(data) = serde_json::from_str::<serde_json::Value>(&text){
                            if let Some(msg_type) = data.get("type").and_then(|v| v.as_str()){
                                match msg_type{
                                    MESSAGE_TYPE_CHAIN => {
                                        if let Some(chain_val) = data.get("chain"){
                                            // if let Ok(new_chain) = serde_json::from_value::<Vec<Block>>(chain_val.clone()) {
                                            //     let mut blockchain = self_clone.blockchain.lock().await;
                                            //     blockchain.replace_chain(new_chain);
                                            //     println!("called replace chain at connect socket");
                                            // }
                                            match serde_json::from_value::<Vec<Block>>(chain_val.clone()) {
                                                Ok(new_chain) => {
                                                    let mut blockchain = self_clone.blockchain.lock().await;
                                                    blockchain.replace_chain(new_chain);
                                                    println!("called replace chain at connect socket");
                                                }
                                                Err(e) => {
                                                    eprintln!("Failed to decode blockchain: {}", e);
                                                    eprintln!("Data was: {:#}", chain_val);
                                                }
                                            }
                                        }
                                    }
                                    MESSAGE_TYPE_TRANSACTION => {
                                        if let Some(tx_val) = data.get("transaction"){
                                            if let Ok(transaction) = serde_json::from_value::<Transaction>(tx_val.clone()){
                                                let mut tp = self_clone.transaction_pool.lock().await;
                                                tp.update_or_add_transaction(transaction);
                                            }
                                        }
                                    }
                                    MESSAGE_TYPE_CLEAR_TRANSACTIONS => {
                                        let mut tp = self_clone.transaction_pool.lock().await;
                                        tp.clear();
                                        println!("Transaction pool is cleared now");
                                    }
                                    _=>{}
                                }
                            }
                        }
                    }
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("WebSocket error: {}", e);
                        break;
                    }
                }
            }
            println!("Read loop ended for peer socket.");
        });

        // Save the write half handle so we can send messages to this socket later
        let peer_socket = PeerSocket { write };

        self.sockets.lock().await.push(peer_socket.clone());

        // Send current blockchain on new connection
        self.send_chain_to_socket(peer_socket.clone()).await;
    }

    pub async fn broadcast_transaction(&self,transaction: &Transaction){
        let tx_message = serde_json::json!({
            "type":MESSAGE_TYPE_TRANSACTION,
            "transaction":transaction
        })
        .to_string();

        let sockets = self.sockets.lock().await;
        let futures = sockets.iter().map(|peer| {
            let msg = tx_message.clone();
            let write = Arc::clone(&peer.write);
            async move{
                let mut locked_writer = write.lock().await;
                let _ =locked_writer.send(Message::Text(msg)).await;
            }
        });
        futures::future::join_all(futures).await;
    }

    async fn send_chain_to_socket(&self, peer: PeerSocket) {
        let blockchain = self.blockchain.lock().await;
        // let chain_json = serde_json::to_string(&blockchain.chain).unwrap();
        // drop(blockchain);
        let chain_json = serde_json::json!({
            "type":MESSAGE_TYPE_CHAIN,
            "chain":&blockchain.chain
        }).to_string();

        let mut writer = peer.write.lock().await;
        if let Err(e) = writer.send(Message::Text(chain_json)).await {
            eprintln!("Failed to send blockchain to socket: {}", e);
        }
    }

    pub async fn sync_chains(&self) {
        println!("sync_chains called");
        let blockchain = self.blockchain.lock().await;
        let chain_message = serde_json::json!({
            "type":MESSAGE_TYPE_CHAIN,
            "chain": &blockchain.chain
        })
        .to_string();
        let sockets = self.sockets.lock().await;
        let futures = sockets.iter().map(|peer| {
            let msg = chain_message.clone();
            let write = Arc::clone(&peer.write);
            async move {
                let mut locked_writer = write.lock().await;
                if let Err(e) = locked_writer.send(Message::Text(msg)).await {
                    eprintln!("Failed to send blockchain to peer socket: {}", e);
                }
            }
        });
        futures::future::join_all(futures).await;
    }

    pub async fn broadcast_clear_transactions(&self){
        let clear_message = serde_json::json!({
            "type":MESSAGE_TYPE_CLEAR_TRANSACTIONS
        }).to_string();

        let sockets = self.sockets.lock().await;
        let futures = sockets.iter().map(|peer| {
            let msg = clear_message.clone();
            let write = Arc::clone(&peer.write);
            async move {
                let mut locked_writer = write.lock().await;
                if let Err(e) = locked_writer.send(Message::Text(msg)).await{
                    eprintln!("failed to send clear transaction message to peer socket: {}",e);
                }
            }
        });
        futures::future::join_all(futures).await;
    }


}
