pub const PROTOCOL_NAME: &str = "PEERTRUST-ASSIGNMENT";

use libp2p::request_response::Codec;
use libp2p::request_response::Behaviour;
use libp2p::request_response::Config;
use libp2p::Transport;
use libp2p::request_response::Event;
use librustysigs::UserCertificate;

pub struct ProtocolName(pub String);

pub enum PEERTRUST_REQUEST {
    BOOTSTRAP,
    ASSIGN,

}

pub struct PeerTrustBootstrapRequest {
    // Certificate For Assignment
    certificate: UserCertificate,
    request_role: RequestRole,
}

pub struct RequestRole {
    authority: String, // ShulginSigning Certificate Hash
    // Add MiniPoW
}