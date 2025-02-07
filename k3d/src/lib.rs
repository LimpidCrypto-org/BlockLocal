use std::{
    marker::PhantomData,
    process::{Command, Output},
};

pub mod cluster;
mod errors;
pub mod node;

use cluster::K3dCluster;
pub use errors::Error;
use errors::Result;
use node::K3dNode;

#[derive(Debug)]
pub struct K3dNoCmd;

#[derive(Debug)]
pub struct K3d<T = K3dNoCmd> {
    cmd: Command,
    t: PhantomData<T>,
}

pub trait K3dGlobalFlags<T> {
    fn timestamps(self, timestamps: bool) -> K3d<T>;
    fn trace(self, trace: bool) -> K3d<T>;
    fn verbose(self, verbose: bool) -> K3d<T>;
}

pub trait K3dCmd {
    fn cluster(self) -> K3d<K3dCluster>;
    fn node(self) -> K3d<K3dNode>;
}

pub trait K3dHelp<T> {
    fn help(self) -> Result<K3d<T>>;
}

pub trait K3dRun<'a> {
    fn run(&'a mut self) -> Result<()>;
    fn check_for_fatal_errors(output: Output) -> Result<()> {
        if !output.status.success() {
            let stderr = String::from_utf8(output.stderr)?;

            Err(Error::CommandFailed(stderr))
        } else {
            Ok(())
        }
    }
}

impl K3d<K3dNoCmd> {
    pub fn new() -> Self {
        Self {
            cmd: Command::new("k3d"),
            t: PhantomData,
        }
    }
}

impl<T> K3dHelp<T> for K3d<T> {
    fn help(mut self) -> Result<K3d<T>> {
        self.cmd.arg("help");

        Ok(self)
    }
}

impl<T> K3dGlobalFlags<T> for K3d<T> {
    fn timestamps(mut self, timestamps: bool) -> K3d<T> {
        self.cmd.arg("--timestamps").arg(timestamps.to_string());

        self
    }

    fn trace(mut self, trace: bool) -> K3d<T> {
        self.cmd.arg("--trace").arg(trace.to_string());

        self
    }

    fn verbose(mut self, verbose: bool) -> K3d<T> {
        self.cmd.arg("--verbose").arg(verbose.to_string());

        self
    }
}

impl K3dCmd for K3d<K3dNoCmd> {
    fn cluster(mut self) -> K3d<K3dCluster> {
        self.cmd.arg("cluster");

        K3d {
            cmd: self.cmd,
            t: PhantomData,
        }
    }

    fn node(mut self) -> K3d<K3dNode> {
        self.cmd.arg("node");

        K3d {
            cmd: self.cmd,
            t: PhantomData,
        }
    }
}
