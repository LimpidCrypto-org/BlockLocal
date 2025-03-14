use crate::{K3d, K3dRun, Result};

pub struct K3dClusterStop;

pub trait K3dClusterStopArgs {
    fn all(self, all: bool) -> K3d<K3dClusterStop>;
}

impl K3dClusterStopArgs for K3d<K3dClusterStop> {
    fn all(mut self, all: bool) -> K3d<K3dClusterStop> {
        self.cmd.arg("--all").arg(all.to_string());

        self
    }
}

impl<'a> K3dRun<'a> for K3d<K3dClusterStop> {
    async fn run(&'a mut self) -> Result<()> {
        let output = self.cmd.spawn()?.wait_with_output().await?;
        Self::check_for_fatal_errors(output)?;

        Ok(())
    }
}
