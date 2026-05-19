use libp2p::PeerId;
use blake2_rfc::blake2s::Blake2s;
use slugencode::{SlugEncodingUsage, errors::SlugEncodingError};

pub mod labels;

pub struct Utils;

impl Utils {
    pub fn new() {

    }
    /// # Create Swarm ID
    /// 
    /// Generates SWARM ID from Peer ID. Used in future applications.
    pub fn create_swarm_id(id: PeerId) -> Result<String, SlugEncodingError> {
        let bytes = id.to_bytes();
        let mut hash = Blake2s::new(6);
        hash.update(&bytes);

        let x = hash.finalize().as_bytes().to_owned();
        let encoder = SlugEncodingUsage::new(slugencode::SlugEncodings::Base58);
        let result = encoder.encode(x);
        return result
    }
}