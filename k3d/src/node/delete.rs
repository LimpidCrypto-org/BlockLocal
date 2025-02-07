use crate::{K3d, K3dRun, Result};

pub struct K3dNodeDelete;

pub trait K3dNodeDeleteArgs {
    fn all(self, all: bool) -> K3d<K3dNodeDelete>;
    fn registries(self, registries: bool) -> K3d<K3dNodeDelete>;
}

impl K3dNodeDeleteArgs for K3d<K3dNodeDelete> {
    fn all(mut self, all: bool) -> K3d<K3dNodeDelete> {
        if all {
            self.cmd.arg("--all");
        }

        self
    }

    fn registries(mut self, registries: bool) -> K3d<K3dNodeDelete> {
        if registries {
            self.cmd.arg("--registries");
        }

        self
    }
}

impl<'a> K3dRun<'a> for K3d<K3dNodeDelete> {
    fn run(&'a mut self) -> Result<()> {
        let output = self.cmd.spawn()?.wait_with_output()?;
        Self::check_for_fatal_errors(output)?;

        Ok(())
    }
}
