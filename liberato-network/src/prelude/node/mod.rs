pub use crate::app;
pub use crate::networking::internals::behavior::MuscarineBehaviourEvent;

/// # MuscarineNode
/// 
/// This is the general implementation of MuscarineNetwork.
pub struct MuscarineNode;

impl MuscarineNode {
    pub fn new() -> Result<(), Box<dyn std::error::Error>> {
        app::main::<MuscarineBehaviourEvent>()
    }
}

#[test]
fn execute() {
    let x = MuscarineNode::new();
}