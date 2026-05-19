pub mod services;

pub trait Attributes {
    /// Gets all the labels that are attributed to this entity
    fn get_labels(&self) -> Vec<String>;
    /// Checks if a specific label is attributed to this entity
    fn contains_label(&self, label: &str) -> bool {
        self.get_labels().iter().any(|l| l == label)
    }
    //fn attribute_extension(&mut self, label: &str, extension: &str);
}

pub trait InterconnectedDesignEssentials {
    /// Gets the registered source ID
    fn get_registered_source_id(&self) -> u64;
    /// Gets the registered checksum
    fn get_registered_checksum(&self) -> String;
}