use crate::errors::Result;

/// Defines the behavior of a command that can be executed by the BlockSim CLI
pub trait BlockSimCommand {
    /// Handles the execution of the command based on the passed arguments
    fn execute(&self) -> Result<()>;
}
