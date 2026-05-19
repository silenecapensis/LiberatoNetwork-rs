/// # Shrine Network API
/// 
/// Shrine Network Utility
pub struct ShrineNetworkAPI;

impl ShrineNetworkAPI {
    pub fn connect(connection: ShrineConnectionConfig) {

    }
    fn bootstrap() {

    }
}

pub enum ShrineConnectionStatus {
    Connected,
    Disconnected,
}

pub enum ShrineConnectionConfig {
    Bootstrap,
    Normal,
    Strict,
    Custom(u16),
}

/// # Bootstrap Methods
/// 
/// SignedCertificate: Uses a signed certificate to gather node information.
/// 
/// Gateway: Uses a gateway
pub enum BootstrapMethod {
    SignedCertificate,
    Gateway,
    DynamicGateway,
    Bridged,
}