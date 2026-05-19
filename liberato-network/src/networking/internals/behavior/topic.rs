// TODO: Add on more hashers
use libp2p::gossipsub::Sha256Topic;
use fixedstr::str256;
use std::str::FromStr;


pub const DEFAULT_TOPIC: &str = "LiberatoEcosystemNetworkv1";

/// # PeyoteTopic
/// 
/// The struct that contains the topic for gossipsub.
#[derive(Clone, Copy, PartialEq, Hash, Debug)]
pub struct PeyoteTopic {
    input: str256,
}

impl PeyoteTopic {
    /// # As Topic
    /// 
    /// Converts to a SHA256 Topic
    /// 
    /// ```rust
    /// fn main() {
    ///     let x = PeyoteTopic::default();
    ///     let topic = x.as_topic();
    /// }
    /// ```
    pub fn as_topic(&self) -> Sha256Topic {
        Sha256Topic::new(self.input.to_str())
    }
}

impl Default for PeyoteTopic {
    fn default() -> Self {
        return Self {
            input: str256::from_str(DEFAULT_TOPIC).unwrap()
        }
    }
}