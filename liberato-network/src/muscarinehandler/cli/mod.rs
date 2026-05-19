//! # MuscarineHandler CLI
//! 
//! This module contains the command line interface.
//! 
//! ## P2P Muscarine
//! 
//! - [ ] Protocols
//!     - [ ] ZeroAuth: A decentralized Authentication Protocol and Assignment
//! 
//! - [ ] Basic Commands
//!     - [ ] ls
//!         - [ ] ls p : get peers
//!         - [ ] ls f : get filesystem
//!     - [ ] retv (retrieve)
//!         - [ ] retv <multiaddr>
//!         - [ ] retv <>

pub struct MuscarineCommandCli(pub MuscarineCommands);

pub enum MuscarineCommands {
    ls,
    lsp, // list peers
    retv,
}