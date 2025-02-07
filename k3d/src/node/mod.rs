use std::marker::PhantomData;

use create::K3dNodeCreate;
use delete::K3dNodeDelete;
use start::K3dNodeStart;
use stop::K3dNodeStop;

use crate::K3d;

pub mod create;
pub mod delete;
pub mod start;
pub mod stop;

#[derive(Debug)]
pub struct K3dNode;

pub trait K3dClusterCmd {
    fn build_name(name: &str) -> String {
        format!("k3d-{}", name)
    }
    fn create(self, name: &str) -> K3d<K3dNodeCreate>;
    fn delete(self, node: &str) -> K3d<K3dNodeDelete>;
    fn start(self, node: &str) -> K3d<K3dNodeStart>;
    fn stop(self, node: &str) -> K3d<K3dNodeStop>;
}

impl K3dClusterCmd for K3d<K3dNode> {
    fn create(mut self, name: &str) -> K3d<K3dNodeCreate> {
        self.cmd.arg("create").arg(name);

        K3d {
            cmd: self.cmd,
            t: PhantomData,
        }
    }

    fn delete(mut self, clusters: &str) -> K3d<K3dNodeDelete> {
        self.cmd.arg("delete").arg(clusters);

        K3d {
            cmd: self.cmd,
            t: PhantomData,
        }
    }

    fn start(mut self, clusters: &str) -> K3d<K3dNodeStart> {
        self.cmd.arg("start").arg(clusters);

        K3d {
            cmd: self.cmd,
            t: PhantomData,
        }
    }

    fn stop(mut self, clusters: &str) -> K3d<K3dNodeStop> {
        self.cmd.arg("stop").arg(clusters);

        K3d {
            cmd: self.cmd,
            t: PhantomData,
        }
    }
}
