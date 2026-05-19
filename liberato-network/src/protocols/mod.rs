//! # Core-Protocols
//! 
//! ## Core
//! 
//! - [ ] Decentralized Chain Access
//!     - [ ] Verifying Nodes (store chain data)
//! - [ ] OintWeb 3.20
//!     - [ ] Serve Logic Through DAG (Block Lattice) In SIP-02
//!     - [ ] Serve Static Assets Through `SIP-05`
//! 
//! 
//! 
//! 
//! ## Possible
//! - [ ] Torrent Protocol
//! - [ ] File Exchange Protocol
//! - [ ] Universal-ICD Protocol
//! - [ ] Storage Protocol
//! - [ ] Chain Request Protocol
//! - [ ] MIP-22 Protocol (TBD) (Muscarine Improvement Proposal 22: Decentralized Storage Standard)
//! - [ ] MIP-23 Protocol (TBD) (Muscarine Improvement Proposal 23: Decentralized Identity Standard)
//! - [ ] MIP-24 Protocol (TBD) (Muscarine Improvement Proposal 24: Decentralized Messaging Standard)
//! - [ ] MIP-25 Protocol (TBD) (Muscarine Improvement Proposal 25: Decentralized Computation Standard)
//! - [ ] MIP-26 Protocol (TBD) (Muscarine Improvement Proposal 26: Decentralized Governance Standard)
//! - [ ] MIP-27 Protocol (TBD) (Muscarine Improvement Proposal 27: Decentralized Finance Standard)
//! - [ ] MIP-28 Protocol (TBD) (Muscarine Improvement Proposal 28: Decentralized Oracles Standard)
//! - [ ] MIP-29 Protocol (TBD) (Muscarine Improvement Proposal 29: Decentralized Identity Verification Standard)
//! - [ ] MIP-30 Protocol (TBD) (Muscarine Improvement Proposal 30: Decentralized Data Storage Standard)
//! - [ ] MIP-31 Protocol (TBD) (Muscarine Improvement Proposal 31: Decentralized Application Standard)
//! - [ ] MIP-32 Protocol (TBD) (Muscarine Improvement Proposal 32: Decentralized Network Standard)
//! - [ ] MIP-33 Protocol (TBD) (Muscarine Improvement Proposal 33: Decentralized Security Standard)
//! - [ ] MIP-34 Protocol (TBD) (Muscarine Improvement Proposal 34: Decentralized Privacy Standard)
//! - [ ] MIP-35 Protocol (TBD) (Muscarine Improvement Proposal 35: Decentralized Interoperability Standard)
//! - [ ] MIP-36 Protocol (TBD) (Muscarine Improvement Proposal 36: Decentralized Scalability Standard)
//! - [ ] MIP-37 Protocol (TBD) (Muscarine Improvement Proposal 37: Decentralized Performance Standard)
//! - [ ] MIP-38 Protocol (TBD) (Muscarine Improvement Proposal 38: Decentralized Reliability Standard)
//! - [ ] MIP-39 Protocol (TBD) (Muscarine Improvement Proposal 39: Decentralized Usability Standard)
//! - [ ] MIP-40 Protocol (TBD) (Muscarine Improvement Proposal 40: Decentralized Accessibility Standard)
//! - [ ] MIP-41 Protocol (TBD) (Muscarine Improvement Proposal 41: Decentralized Maintainability Standard)
//! - [ ] MIP-42 Protocol (TBD) (Muscarine Improvement Proposal 42: Decentralized Extensibility Standard)
//! 
//! ## Liberato
//! 
//! - [ ] 07: Pull-From-Chain

/// Universal-ICD Proto
pub mod universal_icd_protocol;

/// Storage Proto
pub mod storage;

/// Chain Request Proto
pub mod chain_request;

pub mod liberato_protocols;
pub mod muscarine_protocols;

pub mod essential;