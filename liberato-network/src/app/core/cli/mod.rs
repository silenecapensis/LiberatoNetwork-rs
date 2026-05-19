use clap::Parser;

#[derive(Parser)]
pub struct MuscarineCliInput {
    /// CMD Name
    pub name: String,
}

pub struct MuscarineCliParser;

impl MuscarineCliParser {
    pub fn read(cmd: &str) {

    }
}