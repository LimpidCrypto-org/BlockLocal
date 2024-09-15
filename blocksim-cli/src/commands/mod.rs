pub mod errors;
mod get;
mod start;
mod stop;

use get::args::GetArgs;
use start::args::StartArgs;
use stop::args::StopArgs;

use clap::Subcommand;

use crate::{core::blocksim_command::BlockSimCommand, errors::Result};

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Start the blocksim server
    Start(StartArgs),
    /// Stop the blocksim server
    Stop(StopArgs),
    /// Get information about the blocksim server
    Get(GetArgs),
}

impl BlockSimCommand for Commands {
    fn execute(&self) -> Result<()> {
        match self {
            Commands::Start(start_args) => {
                start_args.execute()?;
            }
            Commands::Stop(stop_args) => {
                stop_args.execute()?;
            }
            Commands::Get(get_args) => {
                get_args.execute()?;
            }
        }

        Ok(())
    }
}
