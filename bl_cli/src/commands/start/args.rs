use std::process::{Command, Output};

use clap::Parser;

use crate::{
    commands::{errors::CommandError, k3d::args::K3DArgs},
    core::{
        bl_command::{BLCommand, BLCommandFn},
        bl_output::BLOutput,
    },
    errors::Result,
};

#[derive(Parser, Debug)]
pub struct StartArgs {}

impl StartArgs {
    pub fn start_blocklocal_server() -> Result<BLOutput> {
        let mut k3d_cli: K3DArgs = [
            "cluster",
            "create",
            "blocklocal",
            "--api-port",
            "6550",
            "--port",
            "80:80@loadbalancer",
            "--port",
            "443:443@loadbalancer",
            "--agents",
            "1",
        ]
        .as_slice()
        .into();

        k3d_cli.run()
    }

    pub fn add_blocklocal_redis_node() -> Result<BLOutput> {
        let mut k3d_cli: K3DArgs = [
            "node",
            "create",
            "blocklocal",
            "redis",
            "--role",
            "agent",
            "--label",
            "blocklocal.redis=true",
        ]
        .as_slice()
        .into();

        k3d_cli.run()
    }
}

impl BLCommandFn for StartArgs {
    fn run(&mut self) -> Result<BLOutput> {
        StartArgs::start_blocklocal_server()
    }
}
