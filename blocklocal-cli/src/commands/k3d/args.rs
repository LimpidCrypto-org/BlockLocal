use clap::Parser;

use crate::{
    core::{
        bl_command::{BLCommand, BLCommandFn},
        bl_output::BLOutput,
        errors::BLOutputError,
    },
    errors::{BLError, Result},
};

#[derive(Debug, Parser)]
pub struct K3DArgs {
    #[arg()]
    args: Vec<String>,
}

impl From<&[&str]> for K3DArgs {
    fn from(args: &[&str]) -> Self {
        K3DArgs {
            args: args.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl BLCommandFn for K3DArgs {
    fn run(&mut self) -> Result<BLOutput> {
        let cmd = ("k3d".to_string() + " " + &self.args.join(" ")).into();
        let mut command = BLCommand::new(cmd);
        match command.run() {
            Ok(output) => Ok(output),
            Err(BLError::OutputError(BLOutputError::StdErr(e))) => {
                Ok(BLOutput::try_from(BLOutputError::StdErr(e))?)
            }
            Err(e) => Err(e),
        }
    }
}
