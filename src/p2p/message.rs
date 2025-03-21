use serde::{Serialize, Deserialize};
use crate::blockchain::block::Block;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum MessageData {
  // Misc
  Chat {
    message: String,
  },
  // Peers
  PeerDiscovery {},
  PeerGossip {
    peers: Vec<String>
  },
  // Blockchain
  BlockchainRequest {},
  BlockchainReply {
    chain: Vec<Block>,
  },
  BlockchainTx {
    block: Block,
  },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message
{
    pub sender: String,
    pub payload: MessageData,
}
