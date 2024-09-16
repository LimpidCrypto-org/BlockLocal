pub mod errors;
// mod get;
// mod create;
mod k3d;
mod start;
// mod stop;

// use get::args::GetArgs;
// use start::args::StartArgs;
// use stop::args::StopArgs;

use clap::Subcommand;
use k3d::args::K3DArgs;
use start::args::StartArgs;

use crate::{
    core::{bl_command::BLCommandFn, bl_output::BLOutput},
    errors::Result,
};

#[derive(Debug, Subcommand)]
pub enum Commands {
    K3D(K3DArgs),
    /// Start the blocksim server
    Start(StartArgs),
    // /// Stop the blocksim server
    // Stop(StopArgs),
    // /// Get information about the blocksim server
    // Get(GetArgs),
}

impl BLCommandFn for Commands {
    fn run(&mut self) -> Result<BLOutput> {
        match self {
            Commands::K3D(k3d_args) => k3d_args.run(),
            Commands::Start(start_args) => start_args.run(), // Commands::Stop(stop_args) => {
                                                             //     stop_args.run()?;
                                                             // }
                                                             // Commands::Get(get_args) => {
                                                             //     get_args.run()?;
                                                             // }
        }
    }
}
