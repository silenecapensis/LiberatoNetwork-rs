//! # ChainIndex
//! 
//! ChainIndex is a decentralized way of storing chain data and registering chains.

use fixedstr::{str64, str256};
use libp2p::swarm::StreamProtocol;

const PROTOCOL: StreamProtocol = StreamProtocol::new("/liberato/chain-index/1.0.0");


pub struct ChainIndex {

}

pub struct ChainId {
    chain_symbol: char,
    chain_common_name: str256,
    chain_unique_name: str256,

    chain_id: u64,

    chain_tags: Vec<str64>,

    chain_creator: str256,
    chain_creator_address: str256,
    chain_governance_address: str256,
}



pub struct Request {

}