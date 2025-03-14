use std::path::{Path, PathBuf};

use k3d::{
    cluster::{
        create::{K3dClusterCreate, K3dClusterCreateArgs},
        K3dNodeCmd,
    },
    node::{
        create::{K3dNodeCreate, K3dNodeCreateArgs},
        K3dClusterCmd,
    },
    K3d, K3dCmd, K3dRun,
};
use url::Url;

use crate::errors::Result;

#[derive(Debug)]
pub struct Blockchain {
    name: String,
    http_api: Option<Url>,
    ws_api: Option<Url>,
}

pub trait BlockchainCluster {
    async fn create(&self, config: Option<PathBuf>) -> Result<()>;
    async fn start(&self) -> Result<()>;
    async fn stop(&self) -> Result<()>;
    async fn delete(&self) -> Result<()>;
}

pub trait BlockchainNode {
    async fn create(&self, name: &str, role: &str, config: Option<PathBuf>) -> Result<()>;
    async fn start(&self, name: &str) -> Result<()>;
    async fn stop(&self, name: &str) -> Result<()>;
    async fn delete(&self, name: &str) -> Result<()>;
}

impl Blockchain {
    pub fn new(
        name: String,
        docker_image: Url,
        http_api: Option<Url>,
        ws_api: Option<Url>,
    ) -> Self {
        Self {
            name,
            docker_image,
            http_api,
            ws_api,
        }
    }
}

impl BlockchainCluster for Blockchain {
    async fn create(&self, config: Option<PathBuf>) -> Result<()> {
        let mut k3d: K3d<K3dClusterCreate> = K3d::new().cluster().create(&self.name);
        if let Some(config) = config {
            Ok(k3d
                .k3s_arg(&format!("--config={}", config.to_str().unwrap()))
                .run()
                .await?)
        } else {
            Ok(k3d.run().await?)
        }
    }

    async fn start(&self) -> Result<()> {
        Ok(K3d::new()
            .cluster()
            .start(vec![&K3d::build_name(&self.name)])
            .run()
            .await?)
    }

    async fn stop(&self) -> Result<()> {
        Ok(K3d::new()
            .cluster()
            .stop(vec![&K3d::build_name(&self.name)])
            .run()
            .await?)
    }

    async fn delete(&self) -> Result<()> {
        Ok(K3d::new()
            .cluster()
            .delete(vec![&K3d::build_name(&self.name)])
            .run()
            .await?)
    }
}

impl BlockchainNode for Blockchain {
    async fn create(&self, name: &str, role: &str, config: Option<PathBuf>) -> Result<()> {
        let k3d: K3d<K3dNodeCreate> = K3d::new().node().create(&name);
        if let Some(config) = config {
            Ok(k3d
                .k3s_arg(&format!("--config={}", config.to_str().unwrap()))
                .run()
                .await?)
        } else {
            Ok(k3d.cluster(&self.name).role(role).run().await?)
        }
    }

    async fn start(&self, name: &str) -> Result<()> {
        Ok(K3d::new()
            .node()
            .start(&K3d::build_name(&format!("{}-{}", self.name, name)))
            .run()
            .await?)
    }

    async fn stop(&self, name: &str) -> Result<()> {
        Ok(K3d::new()
            .node()
            .stop(&K3d::build_name(&format!("{}-{}", self.name, name)))
            .run()
            .await?)
    }

    async fn delete(&self, name: &str) -> Result<()> {
        Ok(K3d::new()
            .node()
            .delete(&K3d::build_name(&format!("{}-{}", self.name, name)))
            .run()
            .await?)
    }
}
