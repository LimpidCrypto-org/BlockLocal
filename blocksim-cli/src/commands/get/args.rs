use std::{
    os::unix::process::CommandExt,
    process::{Command, Output},
};

use clap::Parser;

use crate::{
    commands::errors::CommandError, core::blocksim_command::BlockSimCommand, errors::Result,
};

use super::subcommands::GetCommands;

#[derive(Debug, Parser)]
pub struct GetArgs {
    #[command(subcommand)]
    pub command: GetCommands,
}

impl GetArgs {
    pub fn get_k3s_nodes() -> Result<()> {
        match Command::new("sudo")
            .arg("k3s")
            .arg("kubectl")
            .arg("get")
            .arg("nod")
            .output()
        {
            Ok(output) => {
                println!("{}", String::from_utf8_lossy(&output.stdout));
                Ok(())
            }
            Err(error) => return Err(CommandError::ClapError(error.into()).into()),
        }
    }
}

impl BlockSimCommand for GetArgs {
    fn execute(&self) -> Result<()> {
        match &self.command {
            GetCommands::Nodes => GetArgs::get_k3s_nodes()?,
        }

        Ok(())
    }
}
