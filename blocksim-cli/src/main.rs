mod commands;
mod core;
mod errors;
mod utils;

use core::blocksim_command::BlockSimCommand;

use clap::Parser;
use commands::Commands;
use errors::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    cli.command.execute()?;

    Ok(())
}

#[derive(Parser, Debug)]
#[command(name = "blocksim")]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
