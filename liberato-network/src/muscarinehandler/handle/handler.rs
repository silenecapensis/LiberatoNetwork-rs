//! # Muscarine-Handler
//! 
//! Handles input/events
//! 
//! ## Outline
//! 
//! ### Event Handler
//! 
//! - [ ] MuscarineEventHandler: Takes as input an event
//! - [ ] MuscarineEventAction: Scriptable actions on event
//! - [ ] MuscarineEventStateManager: Handles state of data like a merkle-tree
//! 
//! #### Persistence and Storage
//! 
//! - [ ] MuscarineEphermalState: An ephermal state that is distributed.
//! 
//! 
//! - [ ] MuscarineProtocolSlab: Contains the protocol(s) being used.


use libp2p::PeerId;
use libp2p::Swarm;
use libslug::slugcrypt::internals::signature::shulginsigning::ShulginKeypair;

use crate::networking::internals::behavior::MuscarineBehaviour;
use crate::networking::internals::behavior::MuscarineBehaviourEvent;
use crate::muscarinehandler::nodes::NodeTypes;
use crate::networking::internals::keys::P2PKeys;

use libslug::slugcrypt::internals::signature::shulginsigning;

use libslug::slugcrypt::internals::signature::esphand_signature::EsphandKeypair;

use serde::{Serialize,Deserialize};

use libp2p::gossipsub::Sha256Topic;
/// # Bootstrap Address
/// 
/// Gossipsub Address for Bootstrapping using SHA256Topic
pub const BOOTSTRAP: &str = "liberato/bootstrap-muscarine-v1";

use log;

use fixedstr::str256;



/// # MuscarineEventHandler
/// 
/// Version: 0.1.0
/// 
/// Handles Events according to its node_id.
pub struct MuscarineEventHandler {
    version: u16,
    
    node_id: NodeTypes, // handles how they interact
    localstate: LocalState,
    ephermal_state: Option<EphermalStateLinks>,
    slab: Option<SlabData>,
    
    
    // Swarm + Key
    swarm: Swarm<MuscarineBehaviour>,
    key: P2PKeys,
}

pub struct SlabData {

}

pub struct LocalState {
    // Gathers Addresses on Bootstrapping
    pub gossipsub_gather_multiaddr: bool,
}

pub struct EphermalStateLinks {

}

#[derive(Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct BootstrapData {
    /// Ephermal Peer Address
    ephermal_peer_address: PeerId,
    /// Certificate
    certificate: str256, // ShulginSigning
}

impl BootstrapData {
    pub fn parse_cert(&self) {
        let pk = ShulginKeypair::from_x59_pk_format(self.certificate);
    }
    pub fn generate_identity() {
        let keypair = ShulginKeypair::generate();
    }
}


pub struct MuscarineEventHandlerSlab;

impl MuscarineEventHandler {
    pub fn new(&mut self, event: MuscarineBehaviourEvent) {
        if self.node_id == NodeTypes::Liberato {
            /*
            match event {
                MuscarineBehaviourEvent::Input(String::from("bootstrap")) => {
                    log::info!("Bootstrapping With Peers using GossipSub: Topic: {}",BOOTSTRAP);
                    self.swarm.behaviour_mut().gossipsub.subscribe(&Sha256Topic::new(BOOTSTRAP));
                    self.swarm.behaviour_mut().gossipsub.publish(&Sha256Topic::new(BOOTSTRAP), )
                    self.localstate.gossipsub_gather_multiaddr = true
                }
                MuscarineBehaviourEvent::Input(String::from("connect")) => {

                }
            }
            */
        }
    }
}

pub struct MuscarineEventStateManager;