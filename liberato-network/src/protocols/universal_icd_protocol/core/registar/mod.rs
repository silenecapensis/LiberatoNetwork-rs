pub mod domain;

pub struct RegistarEntry {
    pub _type: RegistarEntryType,
    pub value: String,
}

pub enum RegistarEntryType {
    Domain(u8),
    Chain(u64),
    Shrine,
    
    // ETH + SOL
    ETH(u8, String),
    SOL(u8, String),
}

pub struct RegistarDomain {
    pub name: String,
    pub url: String,
    pub description: String,

}

pub trait RegistarEntryLabel {

}
pub mod data_structures;