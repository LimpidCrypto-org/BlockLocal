use std::marker::PhantomData;

use create::K3dClusterCreate;
use delete::K3dClusterDelete;
use start::K3dClusterStart;
use stop::K3dClusterStop;

use crate::K3d;

pub mod create;
pub mod delete;
pub mod start;
pub mod stop;

#[derive(Debug)]
pub struct K3dCluster;

pub trait K3dClusterCmd {
    fn create(self, name: String) -> K3d<K3dClusterCreate>;
    fn delete(self, clusters: Vec<String>) -> K3d<K3dClusterDelete>;
    fn start(self, clusters: Vec<String>) -> K3d<K3dClusterStart>;
    fn stop(self, clusters: Vec<String>) -> K3d<K3dClusterStop>;
}

impl K3dClusterCmd for K3d<K3dCluster> {
    fn create(mut self, name: String) -> K3d<K3dClusterCreate> {
        self.cmd.arg("create").arg(name);

        K3d {
            cmd: self.cmd,
            t: PhantomData,
        }
    }

    fn delete(mut self, clusters: Vec<String>) -> K3d<K3dClusterDelete> {
        self.cmd.arg("delete").args(clusters);

        K3d {
            cmd: self.cmd,
            t: PhantomData,
        }
    }

    fn start(mut self, clusters: Vec<String>) -> K3d<K3dClusterStart> {
        self.cmd.arg("start").args(clusters);

        K3d {
            cmd: self.cmd,
            t: PhantomData,
        }
    }

    fn stop(mut self, clusters: Vec<String>) -> K3d<K3dClusterStop> {
        self.cmd.arg("stop").args(clusters);

        K3d {
            cmd: self.cmd,
            t: PhantomData,
        }
    }
}
