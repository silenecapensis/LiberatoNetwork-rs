//! # MuscarineNetwork-CLI
//! 
//! This module contains all the commands for MuscarineNetwork.
//! 
//! ## Description
//! 
//! MuscarineNetwork offers Command-Line-Interface usage for users that is both extensible and easy to use. It aims to create a decentralized network where Peer-2-Peer communication can be achieved.

use clap::{Args, Parser, Subcommand, ValueEnum};


/// # Custom Protocol ID
/// 
/// A fixedstr of 256 bytes for calling custom protocols.
type CUSTOM_PROTOCOL_ID = fixedstr::str256;

#[derive(Parser)]
#[command(name = "muscarine-network", version, about, long_about = None)]
#[derive(Debug,Clone,PartialEq,PartialOrd,Hash)]
pub struct MuscarineCommands {
    #[command(subcommand)]
    action: MuscarineActionCommands,
}

#[derive(Debug,Clone,Copy,PartialEq,PartialOrd,Hash)]
pub struct MuscarineCommandHandler {
    action: MuscarineActionCommands,
}

/// # MuscarineActionCommands
/// 
/// ## Commands
/// 
/// - [ ] `ls`: List
/// - [ ] `retv`: Retrieve
/// - [ ] `whoami`: info
/// - [ ] `version`: Version info
/// - [ ] `protocols`: Lists protocols
///     - [ ] 
/// 
/// ## Extension Commands
/// 
/// - [ ] `muscarine`
///     - [ ] `muscarine-keys`
/// 
///     - [ ] `muscarine app <>`
///     - [ ] `muscarine ext <>`
///     - [ ] `muscarine ol-app <>`
///     - [ ] `muscarine`
/// 
/// ## Implemented
/// 
/// - [ ] 
#[derive(Debug,Clone,Copy,PartialEq,PartialOrd,Hash, Args, Subcommand)]
pub enum MuscarineActionCommands {
    ls(MuscarineListCommands),
    retv(MuscarineRetrieveCommands), // retrieve
    whoami(MuscarineWhoAmICommands), // whoami
    version,
    protocols,
    audit(MuscarineAuditCommands), // Audits information
    
    // Muscarine Subcommands
    muscarine(MuscarineSubcommands),

    muscarine_protocols(MuscarineExtensibleProtocol), // muscarine-protocols

    // /ol-apps/
    ol_apps(MuscarineOlApps)
}

#[derive(Debug,Clone,Copy,PartialEq,PartialOrd,Hash,Subcommand,Args)]
pub enum MuscarineSubcommands {
    MuscarineKeys, // muscarine-keys
    MuscarineApps, // muscarine-apps
    MuscarineInterop, // muscarine-interop
    MuscarineBootstrap, // muscarine-bootstrap
    MuscarineProtocol, // muscarine-protocol
    MuscarineServices, // muscarine-services
}

/// # MuscarineExtensibleProtocol
/// 
/// Includes the Standard Library and Custom Protocol Usage
#[derive(Debug,Clone,Copy,PartialEq,PartialOrd,Hash,Subcommand)]
pub enum MuscarineExtensibleProtocol {
    Standard(MuscarineStandardProtocols),
    CustomProtocol(CUSTOM_PROTOCOL_ID),
}

#[derive(Debug,Clone,Copy,PartialEq,PartialOrd,Hash,Parser,ValueEnum)]
pub enum MuscarineStandardProtocols {
    PullFromChain,
}

#[derive(Debug,Clone,Copy,PartialEq,PartialOrd,Hash,Parser,ValueEnum)]
pub enum MuscarineAuditCommands {
    peers,
}

#[derive(Debug,Clone,Copy,PartialEq,PartialOrd,Hash,Parser,ValueEnum)]
pub enum MuscarineOlApps {
    Install,
    Remove,
}

#[derive(Debug,Clone,Copy,PartialEq,PartialOrd,Hash,Parser,ValueEnum)]
pub enum MuscarineListCommands {
    peers, // p: Peers connected to
    files, // f: Files in the sandboxed directory
    trustnet, // tn: TrustNet Node Addresses
    bootstrap, // b: Bootstrapping Nodes
    domains, // dns: Domain Names
    certificates, // cert: Certificates
    protocols, // -proto: Lists protocols
    
    // /ol-apps/
    extensions, // exts: Local Extensions to the protocol
    apps, // apps: Local Apps to the protocol
}

#[derive(Debug,Clone,Copy,PartialEq,PartialOrd,Hash,Parser,ValueEnum)]
pub enum MuscarineRetrieveCommands {
    address, // addr: Retrieves an address
    multiaddr, // multi: Retrieves a MultiAddr
    hash, // h: hash retrieve
}

#[derive(Debug,Clone,Copy,PartialEq,PartialOrd,Hash,Parser,ValueEnum)]
pub enum MuscarineWhoAmICommands {
    info, // i: Information
    user, // u: User information
    slab, // slab: Slab of Info
}