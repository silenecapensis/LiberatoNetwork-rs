use libp2p::{Transport, identity};
use libp2p::core::transport;
use libp2p::PeerId;
use libp2p::core::muxing;
use libp2p::tcp;
use libp2p::noise;
use libp2p::core;
use libp2p::yamux;
use libp2p::websocket;

use libp2p::quic::Config as QuicConfig;

/// # MuscarineV1 Transport Protocols
/// 
/// Includes the following protocols for transport:
/// 
/// - [X] TCP
/// - [X] WebSockets-Secure (WSS)
pub struct MuscarineV1Transport(pub transport::Boxed<(PeerId,muxing::StreamMuxerBox)>);

/// # MuscarineV1 Transport (Quic)
/// 
/// Includes the following protocol for transport:
/// 
/// - [X] Quic
pub struct MuscarineV1TransportQuic(pub libp2p::quic::GenTransport<libp2p::quic::tokio::Provider>);

impl MuscarineV1TransportQuic {
    /// # New QUIC Transport
    /// 
    /// Creates a new QUIC transport for MuscarineV1
    pub fn new(keypair: identity::Keypair) -> Self {
        let output: libp2p::quic::GenTransport<libp2p::quic::tokio::Provider> = create_secure_transport_quic(&keypair);
        return Self(output)
    }
}

impl MuscarineV1Transport {
    /// # New Secure TCP Connection
    /// 
    /// Creates a new secure TCP Connection using Noise
    pub fn new(keypair: identity::Keypair) -> Self {
        let transport: transport::Boxed<(PeerId, muxing::StreamMuxerBox)> = create_secure_transport_tcp(&keypair);

        return Self(transport)
    }
    /// # New WebSockets Secure Connection (WSS)
    /// 
    /// Creates a new secure Websockets Secure Connection using Noise
    pub fn new_wss(keypair: identity::Keypair) -> Self {
        let transport: transport::Boxed<(PeerId, muxing::StreamMuxerBox)> = create_secure_transport_wss(&keypair);

        return Self(transport)
    }
}

fn create_secure_transport_tcp(keypair: &identity::Keypair) -> transport::Boxed<(PeerId, muxing::StreamMuxerBox)> {
    let auth_config: noise::Config = noise::Config::new(&keypair).unwrap();
    let transport: transport::Boxed<(PeerId, muxing::StreamMuxerBox)> = tcp::tokio::Transport::default()
        .upgrade(core::upgrade::Version::V1)  // Use protocol upgrade version 1
        .authenticate(auth_config)    // Add Noise encryption
        .multiplex(yamux::Config::default()) // Multiplexing
        .boxed();

    return transport
}

fn create_secure_transport_wss(keypair: &identity::Keypair) -> core::transport::Boxed<(PeerId, core::muxing::StreamMuxerBox)> {
    // Noise Config
    let auth_config: noise::Config = noise::Config::new(&keypair).unwrap();

    // WebSocket Secure Protocol
    let transport: transport::Boxed<(PeerId, muxing::StreamMuxerBox)> = websocket::WsConfig::new(tcp::tokio::Transport::default())
        .upgrade(core::upgrade::Version::V1)
        .authenticate(auth_config)
        .multiplex(yamux::Config::default())
        .boxed();

    return transport
}

fn create_secure_transport_quic(keypair: &identity::Keypair) -> libp2p::quic::GenTransport<libp2p::quic::tokio::Provider> {
    //let auth_config = noise::Config::new(&keypair).unwrap();

    let transport = libp2p::quic::tokio::Transport::new(QuicConfig::new(&keypair));

    return transport

}

fn create_secure_transport_quic_old(keypair: &identity::Keypair) {
    /*
    let auth_config = noise::Config::new(&keypair).unwrap();

    let transport: libp2p::quic::GenTransport<libp2p::quic::tokio::Provider> = libp2p::quic::tokio::Transport::new(QuicConfig::new(&keypair))
        .upgrade(core::upgrade::Version::V1)
        .authenticate(auth_config)
        .multiplex(yamux::Config::default());

    return transport
     */
}
fn create_secure_transport_quic_new() {
    
}