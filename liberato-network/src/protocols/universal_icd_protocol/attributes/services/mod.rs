/// # ICDServiceAttribute
/// 
/// - [X] get_service_name: Returns a String of the Services Common Name
pub trait ICDServiceAttribute {
    /// Gets the service name
    fn get_service_name(&self) -> String;
    /// Gets the service credentials
    fn get_service_credentials(&self) -> String;
}

pub trait ICDEndpointAttribute: ICDServiceAttribute {
    /// Gets the service endpoint URL
    fn get_service_endpoint(&self) -> String;
    /// Gets the service port
    fn get_service_port(&self) -> u16;
    /// Gets blockchain data
    fn request_blockchain_data(&self) -> String;
}

pub trait ICDServiceInteroperabilityAttribute: ICDServiceAttribute {
    /// Gets the supported protocols
    fn get_supported_protocols(&self) -> Vec<String>;
    /// Checks if a specific protocol is supported
    fn supports_protocol(&self, protocol: &str) -> bool {
        self.get_supported_protocols().iter().any(|p| p == protocol)
    }
    
    fn get_service_endpoint(&self) -> String;
}

pub trait ICDPEERS: ICDServiceAttribute + ICDServiceInteroperabilityAttribute {
    /// Gets the peer ID
    fn get_peer_id(&self) -> String;
    /// Gets the peer addresses
    fn get_peer_addresses(&self) -> Vec<String>;
}