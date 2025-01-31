use crate::{K3d, K3dRun, Result};

pub struct K3dClusterDelete;

pub trait K3dClusterDeleteArgs {
    fn all(self, all: bool) -> Result<K3d<K3dClusterDelete>>;
    fn config(self, config: String) -> Result<K3d<K3dClusterDelete>>;
}

impl K3dClusterDeleteArgs for K3d<K3dClusterDelete> {
    fn all(mut self, all: bool) -> Result<K3d<K3dClusterDelete>> {
        self.cmd.arg("--all").arg(all.to_string());

        Ok(self)
    }

    fn config(mut self, config: String) -> Result<K3d<K3dClusterDelete>> {
        self.cmd.arg("--config").arg(config);

        Ok(self)
    }
}

impl<'a> K3dRun<'a> for K3d<K3dClusterDelete> {
    fn run(&'a mut self) -> Result<()> {
        self.cmd.spawn()?.wait_with_output()?;

        Ok(())
    }
}
