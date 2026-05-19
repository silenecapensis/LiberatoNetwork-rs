//! # Pull-From-Chain
//! 
//! Pull-From-Chain offers a way to get data from chains using P2P connections and storage containers to contain validated information.
//! 
//! ## Chain ID
//! 
//! Supported Chains follow the **Liberato Interoperable Chain List**:
//! 
//! - Chain ID: u64
//! 
//! ## Storage
//! 
//! Storage is done by getting data from chains and storing in a DAG/Block Lattice.
use libp2p::swarm::StreamProtocol;

use fixedstr::str256;

pub struct PullFromChainID;

/// PROTOCOL: /liberato/pullfromchain/1.0.0
const PROTOCOL: StreamProtocol = StreamProtocol::new("/liberato/pullfromchain/1.0.0");

#[derive(Debug,Copy,Clone)]
pub struct Request {
    action: PullFromChainAction,
    chain_id: u64,
    data: str256,
}

#[derive(Debug,Copy,Clone)]
pub enum PullFromChainAction {
    Pull,
    PullFromStrg,
    InitStrg,
}

pub struct Response {

}