use std::collections::HashSet;

use fixedstr::{str64, str96, str128, str192, str256};

/// # Muscaria Index Address
/// 
/// The official address of content.
pub struct MuscariaIndexAddress(pub str256);

pub struct MuscariaIndex {
    entries: HashSet<MuscariaIndexEntry,
}


pub struct MuscariaIndexEntry(pub str256);

pub struct MuscariaIndexEntryMetadata {
    addr: MuscariaIndexEntry,
    
    id: str128,
    
    common_name: Option<str256>,
    description: Option<str256>,
    tags: Option<Vec<str64>>,
    author: Option<str256>,

    version: u32,
    checksum: str192,

    data: Vec<MuscariaIndexAddress>,
}