pub struct Source {
    /// The registered source ID and checksum
    pub registered_id: u64,
    pub registered_checksum: String,

    // Source
    pub git_repo: String,

    // Labels
    pub labels: Vec<String>,
}

pub struct Bridge {
    pub source: Source,
    pub target_source_id: u64,
    pub target_git_repo: String,
}