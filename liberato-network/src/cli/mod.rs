pub struct MuscarineCLI;

/// Core components of CLI
pub mod core;

pub struct MuscarineCommandLineInterface(pub MuscarineCommands)

pub enum MuscarineCommands {
    ls(MuscarineEnumeratedLs),
}

pub enum MuscarineEnumeratedLs {
    peers, // -p
    files, // -f
    domains, // -d
    certificates, // -cert
    trustnetwork, // -trust
}

impl MuscarineCommands {
    pub fn from(&self) -> &str {
        match self {
            Self::ls(x) => {
                let cmd = match x {
                    MuscarineEnumeratedLs::peers => "ls p",
                    MuscarineEnumeratedLs::domains => "ls dn",
                    MuscarineEnumeratedLs::certificates => "ls cert",
                    MuscarineEnumeratedLs::trustnetwork => "ls tn",
                    MuscarineEnumeratedLs::files => "ls f",
                }
                return cmd
            }
        }
    }
}

pub enum MuscarineCLICommand {
    Bootstrap(String),
    CMD(String),
}