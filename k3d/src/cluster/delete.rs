use crate::{K3d, K3dRun, Result};

pub struct K3dClusterDelete;

pub trait K3dClusterDeleteArgs {
    fn all(self, all: bool) -> K3d<K3dClusterDelete>;
    fn config(self, config: &str) -> K3d<K3dClusterDelete>;
}

impl K3dClusterDeleteArgs for K3d<K3dClusterDelete> {
    fn all(mut self, all: bool) -> K3d<K3dClusterDelete> {
        self.cmd.arg("--all").arg(all.to_string());

        self
    }

    fn config(mut self, config: &str) -> K3d<K3dClusterDelete> {
        self.cmd.arg("--config").arg(config);

        self
    }
}

impl<'a> K3dRun<'a> for K3d<K3dClusterDelete> {
    fn run(&'a mut self) -> Result<()> {
        let output = self.cmd.spawn()?.wait_with_output()?;
        Self::check_for_fatal_errors(output)?;

        Ok(())
    }
}
