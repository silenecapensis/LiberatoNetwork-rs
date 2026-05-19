use libp2p::{
    swarm::{
        behaviour::ConnectionEstablished, behaviour::FromSwarm, behaviour::ToSwarm, NetworkBehaviour,
        THandlerInEvent, THandlerOutEvent,
    },
    futures::{AsyncRead, AsyncWrite, FutureExt},
    InboundUpgrade, OutboundUpgrade, PeerId, StreamProtocol,
};
use std::{collections::VecDeque, io};


pub mod _0x00_assignment_protocol;
pub mod _0x01_icd_modular_protocol;
pub mod responseprotocol;



