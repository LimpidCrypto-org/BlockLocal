use k3d::{cluster::K3dClusterCmd, K3dCmd};
use url::Url;

#[derive(Debug)]
pub struct Blockchain {
    name: String,
    docker_image: Url,
    http_api: Option<Url>,
    ws_api: Option<Url>,
}

pub trait BlockchainCluster {
    async fn create(&self) -> Result<()>;
    async fn start(&self) -> Result<()>;
    async fn stop(&self) -> Result<()>;
    async fn delete(&self) -> Result<()>;
}

pub trait BlockchainNode {
    async fn create(&self) -> Result<()>;
    async fn start(&self) -> Result<()>;
    async fn stop(&self) -> Result<()>;
    async fn delete(&self) -> Result<()>;
}

impl BlockchainCluster for Blockchain {
    async fn create(&self) -> Result<()> {
        k3d::K3d::new().cluster().create(self.name).run().await?;
    }

    async fn start(&self) -> Result<()> {
        k3d::K3d::new().cluster().start(self.name).run().await?;
    }

    async fn stop(&self) -> Result<()> {
        k3d::K3d::new().cluster().stop(self.name).run().await?;
    }

    async fn delete(&self) -> Result<()> {
        k3d::K3d::new().cluster().delete(self.name).run().await?;
    }
}

impl BlockchainNode for Blockchain {
    async fn create(&self) -> Result<()> {}
}
