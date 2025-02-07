use crate::{K3d, K3dRun, Result};

pub struct K3dClusterStart;

pub trait K3dClusterStartArgs {
    fn all(self, all: bool) -> K3d<K3dClusterStart>;
    fn timeout(self, timeout: &str) -> K3d<K3dClusterStart>;
    fn wait(self, wait: bool) -> K3d<K3dClusterStart>;
}

impl K3dClusterStartArgs for K3d<K3dClusterStart> {
    fn all(mut self, all: bool) -> K3d<K3dClusterStart> {
        self.cmd.arg("--all").arg(all.to_string());

        self
    }

    fn timeout(mut self, timeout: &str) -> K3d<K3dClusterStart> {
        self.cmd.arg("--timeout").arg(timeout);

        self
    }

    fn wait(mut self, wait: bool) -> K3d<K3dClusterStart> {
        self.cmd.arg("--wait").arg(wait.to_string());

        self
    }
}

impl<'a> K3dRun<'a> for K3d<K3dClusterStart> {
    fn run(&'a mut self) -> Result<()> {
        let output = self.cmd.spawn()?.wait_with_output()?;
        Self::check_for_fatal_errors(output)?;

        Ok(())
    }
}
