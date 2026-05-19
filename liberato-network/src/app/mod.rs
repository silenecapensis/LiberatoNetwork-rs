use libp2p::core::transport::ListenerId;
use libp2p::futures::StreamExt;
use log::{warn,info, trace, debug};
use log::error;
use pretty_env_logger;
use tokio::io::AsyncBufReadExt;
use crate::networking::internals::behavior::{MuscarineBehaviour, MuscarineBehaviourEvent};
use crate::networking::internals::keys;
use crate::networking::internals::swarm::{MuscarineSwarm};

use libp2p::floodsub::Topic;

use libp2p::swarm::SwarmEvent;

use libp2p::{Multiaddr, PeerId, Swarm};

use std::str::FromStr;

use crate::core::internals::Utils;

use libp2p::swarm::ListenAddresses;

use crate::cli::MuscarineCLI;
use crate::cli::MuscarineCLICommand;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use futures::Stream;
//use std::io;

use dotenvy;
pub mod bootstrap;

const swarm_creation: &str = "0x22";
const SHRINDO_TOPIC: &str = "Liberato-v1";
const SHRINDO_TOPIC_BOOTSTRAP: &str = "Liberato-Bootstrap-v1";
const SHRINDO_TOPIC_KEYEXCHANGE: &str = "Liberato-KeyExchange-v1";
const SHRINDO_TOPIC_ESSENTIALS: &str = "Lament-Essentials"; // search for essentials

/// Constants
pub mod constants;
pub mod core;

use crate::app::constants::DEFAULT_P2P;
use crate::networking::internals::behavior::topic::PeyoteTopic;

const local_address: &str = "/ip4/127.0.0.1/tcp/9076";

pub struct ShrindoApp;
use crate::networking::internals::behavior::EventHandler;

#[tokio::main]
pub async fn main<T: EventHandler>() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().unwrap();

    pretty_env_logger::init();
    debug!("Pretty Env Logger Initialized...");

    info!("Generating Ephermal ED25519 Key...");
    // Key Generation
    let key = crate::networking::internals::keys::P2PKeys::generate_ed25519();
    let local_peer_id = key.key.public().to_peer_id();
    info!("Peer-ID: {}", local_peer_id);

    // Swarm
    let mut swarm: libp2p::Swarm<crate::networking::internals::behavior::MuscarineBehaviour> = MuscarineSwarm::new(key);
    info!("Swarm Creation ID: {}", Utils::create_swarm_id(local_peer_id).unwrap());

    //=====BOOTSTRAP CLI=====//


    // Store + Kademelia
    //let store = MemoryStore::new(local_peer_id);
    
    
    // Behaviour
    //swarm.behaviour_mut().floodsub.subscribe(Topic::new(SHRINDO_TOPIC));
    //info!("[Floodsub] Subscribed to {}", SHRINDO_TOPIC);
    //swarm.behaviour_mut().floodsub.subscribe(Topic::new(SHRINDO_TOPIC_BOOTSTRAP));
    //info!("[Floodsub] Subscribed to {}", SHRINDO_TOPIC_BOOTSTRAP);

    //swarm.behaviour_mut().gossipsub.subscribe();

    let mut stdin: io::Lines<io::BufReader<io::Stdin>> = tokio::io::BufReader::new(tokio::io::stdin()).lines(); 
    let id = swarm.listen_on(Multiaddr::from_str(local_address).unwrap())?;

    info!("[Muscarine-Network] Connecting to LiberatoNetwork3.20 with Peer-ID: {}",local_peer_id);

    loop {
        let evt: Option<MuscarineBehaviourEvent> = {
            tokio::select! {
                line = stdin.next_line() => Some(MuscarineBehaviourEvent::Input(line.expect("can get line").expect("can read line"))),
    
                event = swarm.select_next_some() => {
                    info!("Unhandled Swarm Event: {:?}", event);
                    None
                },
            }
        };
        println!("DEBUG: {:?}", evt);
        // Handle evt event
        if evt == Some(MuscarineBehaviourEvent::Input(String::from("ls"))) {
            println!("Listing Peers");
        }
        else {
            println!("Error");
        }
    }

    loop {

            /// # Event
            let event: SwarmEvent<crate::networking::internals::behavior::MuscarineBehaviourEvent> = swarm.select_next_some().await;
        
            //let line = stdin.next_line().await;
            let mut buffer = Vec::new();
            let input = tokio::io::stdin().read(&mut buffer).await;
        
            match event {
                SwarmEvent::NewListenAddr { listener_id, address} => {
                    info!("[Swarm] Listening On {} with listener_id: {}", local_address, listener_id);
                },
                SwarmEvent::Behaviour(mut muscarine_behaviour_event) => {
                    info!("[Swarm] Received Behaviour Event: {:?}", muscarine_behaviour_event.handle_event());
                },
                _ => (),
            }
        //swarm.behaviour_mut().identify.push(peers);
        //let mut cmd = String::new();
        //io::stdin().read_line(&mut cmd)?;
    }

    Ok(())
}

async fn event_handler(event: MuscarineBehaviourEvent) {

}

#[test]
fn run() {
    let x = main::<MuscarineBehaviourEvent>();
}