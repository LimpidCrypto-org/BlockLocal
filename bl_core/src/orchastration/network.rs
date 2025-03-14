use ipnet::IpNet;

use super::image::ImageConfig;

/// The network configuration for managing a network
#[derive(Debug)]
pub struct NetworkConfig {
    name: String,
    subnet: IpNet,
    nodes: Vec<ImageConfig>,
}
