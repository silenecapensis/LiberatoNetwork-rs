use url::Url;

pub struct DomainDefinition {
    common_name: String,
    url: Url,
}


pub struct RepositoryDefinition {
    url: Url,
}

pub struct DatabaseDefinition {
    url: Url,
}