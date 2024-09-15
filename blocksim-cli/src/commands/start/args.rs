use std::process::Command;

use clap::Parser;

use crate::{
    commands::errors::CommandError, core::blocksim_command::BlockSimCommand, errors::Result,
};

#[derive(Parser, Debug)]
pub struct StartArgs {}

impl StartArgs {
    pub fn start_k3s() -> Result<()> {
        if let Err(error) = Command::new("sudo")
            .arg("k3s")
            .arg("server")
            .arg("&")
            .spawn()
        {
            return Err(CommandError::ClapError(error.into()).into());
        }

        Ok(())
    }
}

impl BlockSimCommand for StartArgs {
    fn execute(&self) -> Result<()> {
        StartArgs::start_k3s()
    }
}
