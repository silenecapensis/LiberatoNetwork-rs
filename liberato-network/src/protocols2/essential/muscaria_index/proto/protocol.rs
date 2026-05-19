use super::id::PROTOCOL_ID;
use libp2p::request_response::Codec;
use libp2p::request_response::ProtocolSupport;
use libp2p::core::upgrade::
use async_trait::async_trait;
use futures::prelude::*;

#[derive(Clone, Default)]
pub struct MuscariaIndexCodec;

