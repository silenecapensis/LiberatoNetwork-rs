//! # Swarms
//! 
//! These are the following swarms that are provided for the protocol.
//! 
//! - [X] Core Protocol Swarm (MUSCARINE-SWARM)
//! - [ ] Bootstrap Swarm
//!     - [ ] Bootstraps Nodes using KAD and gateways
//!     - [ ] Port 9670
//! - [ ] Service
//! 

use libp2p::swarm::Config;
use libp2p::{PeerId, Swarm, SwarmBuilder};
use crate::networking::internals::keys::P2PKeys;

use super::behavior::MuscarineBehaviour;
use libp2p::identity;
use super::transport::MuscarineV1Transport;
use super::transport::MuscarineV1TransportQuic;

pub struct MuscarineSwarm;

impl MuscarineSwarm {
    pub fn new(key: P2PKeys) -> Swarm<MuscarineBehaviour> {
        let transport = MuscarineV1Transport::new(key.clone().key);

        let pk = key.clone().key.public();
        let peer_id = pk.to_peer_id();

        let swarm: Swarm<MuscarineBehaviour> = Swarm::new(transport.0,MuscarineBehaviour::new(key.clone()), peer_id, Config::with_tokio_executor());

        return swarm


    }
}

/*
pub struct BootstrapSwarm;

impl BootstrapSwarm {
    pub fn new(key: ShrineKeys) -> Swarm<BootstrapSwarm> {
        let transport = ShrineTransport::new(key.clone().key);

        let pk = key.clone().key.public();
        let peer_id = pk.to_peer_id();

        let swarm = Swarm::new(transport.0,ShrineBehaviour::new(key.clone()), peer_id, Config::with_tokio_executor());

        return swarm
    }
}
    */