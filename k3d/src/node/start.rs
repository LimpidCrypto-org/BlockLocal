use crate::{K3d, K3dRun, Result};

pub struct K3dNodeStart;

impl<'a> K3dRun<'a> for K3d<K3dNodeStart> {
    fn run(&'a mut self) -> Result<()> {
        let output = self.cmd.spawn()?.wait_with_output()?;
        Self::check_for_fatal_errors(output)?;

        Ok(())
    }
}
