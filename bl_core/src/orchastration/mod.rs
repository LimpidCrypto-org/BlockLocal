pub mod image;
pub mod network;

use network::NetworkConfig;

use crate::Result;

pub trait OrchastrationStrategy {
    async fn start_network<'a>(&'a self, name: &'a NetworkConfig) -> Result<()>;
    async fn stop_network<'a>(&'a self, name: &'a NetworkConfig) -> Result<()>;
    async fn delete_network<'a>(&'a self, name: &'a NetworkConfig) -> Result<()>;
    async fn create_node<'a>(&'a self, network: &'a NetworkConfig) -> Result<()>;
}
