use crate::{K3d, K3dRun, Result};

#[derive(Debug)]
pub struct K3dNodeCreate;

pub trait K3dNodeCreateArgs {
    /// Cluster URL or k3d cluster name to connect to. (default "k3s-default")
    fn cluster(self, cluster: &str) -> K3d<K3dNodeCreate>;
    /// Specify k3s image used for the node(s) (default: copied from existing node)
    fn image(self, image: &str) -> K3d<K3dNodeCreate>;
    /// Additional args passed to k3d command
    fn k3s_arg(self, k3s_arg: &str) -> K3d<K3dNodeCreate>;
    /// Specify k3s node labels in format "foo=bar"
    fn k3s_node_label(self, k3s_node_label: &str) -> K3d<K3dNodeCreate>;
    /// Memory limit imposed on the node [From docker]
    fn memory(self, memory: &str) -> K3d<K3dNodeCreate>;
    /// Add node to (another) runtime network
    fn network(self, network: &str) -> K3d<K3dNodeCreate>;
    /// Number of replicas of this node specification. (default 1)
    fn replicas(self, replicas: i32) -> K3d<K3dNodeCreate>;
    /// Specify node role [server, agent] (default "agent")
    fn role(self, role: &str) -> K3d<K3dNodeCreate>;
    /// Specify container runtime labels in format "foo=bar"
    fn runtime_label(self, runtime_label: &str) -> K3d<K3dNodeCreate>;
    /// Specify container runtime ulimit in format "ulimit=soft:hard"
    fn runtime_ulimit(self, runtime_ulimit: &str) -> K3d<K3dNodeCreate>;
    /// Maximum waiting time for '--wait' before canceling/returning.
    fn timeout(self, timeout: &str) -> K3d<K3dNodeCreate>;
    /// Override cluster token (required when connecting to an external cluster)
    fn token(self, token: &str) -> K3d<K3dNodeCreate>;
    /// Wait for the node(s) to be ready before returning. (default true)
    fn wait(self, wait: bool) -> K3d<K3dNodeCreate>;
}

impl K3dNodeCreateArgs for K3d<K3dNodeCreate> {
    fn cluster(mut self, cluster: &str) -> K3d<K3dNodeCreate> {
        self.cmd.arg("--cluster").arg(cluster);

        self
    }

    fn image(mut self, image: &str) -> K3d<K3dNodeCreate> {
        self.cmd.arg("--image").arg(image);

        self
    }

    fn k3s_arg(mut self, k3s_arg: &str) -> K3d<K3dNodeCreate> {
        self.cmd.arg("--k3s-arg").arg(k3s_arg);

        self
    }

    fn k3s_node_label(mut self, k3s_node_label: &str) -> K3d<K3dNodeCreate> {
        self.cmd.arg("--k3s-node-label").arg(k3s_node_label);

        self
    }

    fn memory(mut self, memory: &str) -> K3d<K3dNodeCreate> {
        self.cmd.arg("--memory").arg(memory);

        self
    }

    fn network(mut self, network: &str) -> K3d<K3dNodeCreate> {
        self.cmd.arg("--network").arg(network);

        self
    }

    fn replicas(mut self, replicas: i32) -> K3d<K3dNodeCreate> {
        self.cmd.arg("--replicas").arg(replicas.to_string());

        self
    }

    fn role(mut self, role: &str) -> K3d<K3dNodeCreate> {
        self.cmd.arg("--role").arg(role);

        self
    }

    fn runtime_label(mut self, runtime_label: &str) -> K3d<K3dNodeCreate> {
        self.cmd.arg("--runtime-label").arg(runtime_label);

        self
    }

    fn runtime_ulimit(mut self, runtime_ulimit: &str) -> K3d<K3dNodeCreate> {
        self.cmd.arg("--runtime-ulimit").arg(runtime_ulimit);

        self
    }

    fn timeout(mut self, timeout: &str) -> K3d<K3dNodeCreate> {
        self.cmd.arg("--timeout").arg(timeout);

        self
    }

    fn token(mut self, token: &str) -> K3d<K3dNodeCreate> {
        self.cmd.arg("--token").arg(token);

        self
    }

    fn wait(mut self, wait: bool) -> K3d<K3dNodeCreate> {
        if wait {
            self.cmd.arg("--wait");
        }

        self
    }
}

impl<'a> K3dRun<'a> for K3d<K3dNodeCreate> {
    async fn run(&'a mut self) -> Result<()> {
        let output = self.cmd.spawn()?.wait_with_output().await?;
        Self::check_for_fatal_errors(output)?;

        Ok(())
    }
}
