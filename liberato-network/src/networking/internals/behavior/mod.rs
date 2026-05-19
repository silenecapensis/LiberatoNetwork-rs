//! # Peyote-Network
//! 
//! Peyote Network is a libp2p network for use in distributed/decentralized systems.
//! 
//! ## Features
//! 
//! - [X] Implemented Protocols
//!     - [ ] AUTONAT (Client + Server)
//!     - [ ] GossipSub (Broadcasting Data to a certain SHA256 topic)
//!     - [ ] FloodSub
//!     - [ ] Peer Routing
//!         - [X] Kademelia (DHT)
//!         - [X] MDNS (local network)
//!     

use libp2p::autonat::Behaviour as AutonatBehaviour;
use libp2p::autonat::v2::client::Behaviour as AutonatClient;
use libp2p::autonat::v2::client::Config as AutonatClientConfig;
use libp2p::autonat::v2::server::Behaviour as AutonatServer;
use libp2p::gossipsub::Behaviour as GossipSubBehaviour;
use libp2p::gossipsub::Config as GossipSubConfig;
use libp2p::ping::Event;
#[macro_use(NetworkBehaviour)]
use libp2p::swarm::NetworkBehaviour;


use libp2p::identify::Behaviour as IdentifyBehaviour;
use libp2p::identify::Config as IdentifyConfig;

use libp2p::gossipsub::MessageAuthenticity;

use libp2p::gossipsub::ConfigBuilder as GossipConfigBuilder;
use libp2p::gossipsub::ConfigBuilderError as GossipConfigBuilderError;

use libp2p::request_response::Behaviour as RequestResponseBehaviour;
use libp2p::kad::Behaviour as KademliaBehaviour;
use libp2p::kad::store::MemoryStore;
use libp2p::relay::client::Behaviour as RelayClient;
use libp2p::relay::Behaviour as RelayServer;
use libp2p::relay::Config as RelayServerConfig;
use libp2p::mdns::Behaviour as MdnsBehaviour;

use libp2p::floodsub::Behaviour as FloodsubBehaviour;
use libp2p::floodsub::Config as FloodsubConfig;

use libp2p::ping::Behaviour as PingBehaviour;
use libp2p::ping::Config as PingConfig;

// Must Use
use libp2p::gossipsub::Sha256Topic;

use rand::rngs::OsRng;

use crate::networking::internals::keys::P2PKeys;
use log::info;

pub mod peer_discovery;
pub mod topic;

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "MuscarineBehaviourEvent", event_process = false)]
pub struct MuscarineBehaviour {
    #[behaviour(out_event = "libp2p::autonat::v2::client::Event")]
    pub autonat_client: libp2p::autonat::v2::client::Behaviour,
    #[behaviour(out_event = "libp2p::autonat::v2::server::Event")]
    pub autonat_server: libp2p::autonat::v2::server::Behaviour,

    // Ping (P2P)
    #[behaviour(out_event = "libp2p::ping::Event")]
    pub ping: PingBehaviour,

    // PubSub
    #[behaviour(out_event = "libp2p::gossipsub::Event")]
    pub gossipsub: libp2p::gossipsub::Behaviour,
    //floodsub: FloodsubBehaviour,

    // Identify
    #[behaviour(out_event = "libp2p::identify::Event")]
    pub identify: IdentifyBehaviour,
    
    // Peer Routing
    #[behaviour(out_event = "libp2p::kad::Event")]
    pub kademlia: libp2p::kad::Behaviour<MemoryStore>,

    //pub floodsub: FloodsubBehaviour,

    // Relay
    #[behaviour(out_event = "libp2p::relay::Event")]
    pub relay_server: libp2p::relay::Behaviour,
    //request_response: RequestResponseBehaviour<FileExchangeCodec>,

    //#[behaviour(ignore)]
    
}

impl MuscarineBehaviour {
    pub fn new(key: P2PKeys) -> Self {
        let peer_id = key.key.public().to_peer_id();
        
        let rng = OsRng;


        Self {
            autonat_client: AutonatClient::new(rng, AutonatClientConfig::default()),
            autonat_server: AutonatServer::new(rng),
            
            ping: PingBehaviour::new(PingConfig::default()),
            // TODO: remove unwrap
            gossipsub: GossipSubBehaviour::new(MessageAuthenticity::Signed(key.key.clone()), GossipConfigBuilder::default().validation_mode(libp2p::gossipsub::ValidationMode::Strict).build().unwrap()).unwrap(),
            identify: IdentifyBehaviour::new(IdentifyConfig::new(String::from("Shrindo-Identify"), key.key.clone().public())),
            //floodsub: FloodsubBehaviour::new(key.key.public().to_peer_id()),
            kademlia: KademliaBehaviour::new(key.key.public().to_peer_id(),MemoryStore::new(key.key.public().to_peer_id())), // Create a Memory Store Service For Gathering Peers
            relay_server: RelayServer::new(peer_id, RelayServerConfig::default()),
        }
    }
}

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
pub enum MuscarineBehaviourEvent {
    AutonatClient(libp2p::autonat::v2::client::Event),
    AutonatServer(libp2p::autonat::v2::server::Event),
    Ping(libp2p::ping::Event),
    Gossipsub(libp2p::gossipsub::Event),
    Identify(libp2p::identify::Event),
    Kademlia(libp2p::kad::Event),
    RelayServer(libp2p::relay::Event),
    Input(String),
    ResponseSender(String),
}

impl PartialEq for MuscarineBehaviourEvent {
    fn eq(&self, other: &Self) -> bool {
        match (self,other) {
            (MuscarineBehaviourEvent::Input(a), MuscarineBehaviourEvent::Input(b)) => a == b,
            _ => false
        }
    }
}
/*
impl PartialEq<MuscarineBehaviourEvent> for MuscarineBehaviourEvent {
    fn eq(&self, other: &MuscarineBehaviourEvent) -> bool {
        self == other
    }
    fn ne(&self, other: &MuscarineBehaviourEvent) -> bool {
        self != other
    }
}
*/

impl From<libp2p::autonat::v2::client::Event> for MuscarineBehaviourEvent {
    fn from(event: libp2p::autonat::v2::client::Event) -> Self {
        MuscarineBehaviourEvent::AutonatClient(event)
    }
}

impl From<libp2p::autonat::v2::server::Event> for MuscarineBehaviourEvent {
    fn from(event: libp2p::autonat::v2::server::Event) -> Self {
        MuscarineBehaviourEvent::AutonatServer(event)
    }
}

impl From<libp2p::ping::Event> for MuscarineBehaviourEvent {
    fn from(event: libp2p::ping::Event) -> Self {
        MuscarineBehaviourEvent::Ping(event)
    }
}

impl From<libp2p::gossipsub::Event> for MuscarineBehaviourEvent {
    fn from(event: libp2p::gossipsub::Event) -> Self {
        MuscarineBehaviourEvent::Gossipsub(event)
    }
}

impl From<libp2p::identify::Event> for MuscarineBehaviourEvent {
    fn from(event: libp2p::identify::Event) -> Self {
        MuscarineBehaviourEvent::Identify(event)
    }
}

impl From<libp2p::kad::Event> for MuscarineBehaviourEvent {
    fn from(event: libp2p::kad::Event) -> Self {
        MuscarineBehaviourEvent::Kademlia(event)
    }
}

impl From<libp2p::relay::Event> for MuscarineBehaviourEvent {
    fn from(event: libp2p::relay::Event) -> Self {
        MuscarineBehaviourEvent::RelayServer(event)
    }
}

pub trait EventHandler {
    fn handle_event(&mut self);
}

impl EventHandler for MuscarineBehaviourEvent {
    fn handle_event(&mut self) {
        match self {
            MuscarineBehaviourEvent::AutonatClient(event) => {
                // Handle Autonat Client Event
                info!("[AutonatClient] Received Autonat Client Event: {:?}", event);
            },
            MuscarineBehaviourEvent::AutonatServer(event) => {
                // Handle Autonat Server Event
                info!("[AutonatServer] Received Autonat Server Event: {:?}", event);
            },
            MuscarineBehaviourEvent::Ping(event) => {
                // Handle Ping Event
                info!("[Ping] Received Ping Event: {:?}", event);
            },
            MuscarineBehaviourEvent::Gossipsub(event) => {
                // Handle Gossipsub Event
                info!("[Gossipsub] Received Gossipsub Event: {:?}", event);
            },
            MuscarineBehaviourEvent::Identify(event) => {
                // Handle Identify Event
                info!("[Identify] Received Identify Event: {:?}", event);
            },
            MuscarineBehaviourEvent::Kademlia(event) => {
                // Handle Kademlia Event
                info!("[Kademlia] Received Kademlia Event: {:?}", event);
            },
            MuscarineBehaviourEvent::RelayServer(event) => {
                // Handle Relay Server Event
                info!("[RelayServer] Received Relay Server Event: {:?}", event);
            },
            _ => {
                info!("Not Supported");
            }
        }
    }
}