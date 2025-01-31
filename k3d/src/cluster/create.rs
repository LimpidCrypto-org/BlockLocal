use crate::{K3d, K3dRun, Result};

#[derive(Debug)]
pub struct K3dClusterCreate;

pub trait K3dClusterCreateArgs {
    fn agents(self, agents: u16) -> K3d<K3dClusterCreate>;
    fn agents_memory(self, agents_memory: String) -> K3d<K3dClusterCreate>;
    fn api_port(self, api_port: String) -> K3d<K3dClusterCreate>;
    fn config(self, config: String) -> K3d<K3dClusterCreate>;
    fn env(self, env: String) -> K3d<K3dClusterCreate>;
    fn gpus(self, gpus: String) -> K3d<K3dClusterCreate>;
    fn host_alias(self, host_alias: String) -> K3d<K3dClusterCreate>;
    fn host_pid_mode(self, host_pid_mode: bool) -> K3d<K3dClusterCreate>;
    fn image(self, image: String) -> K3d<K3dClusterCreate>;
    fn k3s_arg(self, k3s_arg: String) -> K3d<K3dClusterCreate>;
    fn k3s_node_label(self, k3s_node_label: String) -> K3d<K3dClusterCreate>;
    fn kubeconfig_switch_context(self, kubeconfig_switch_context: bool) -> K3d<K3dClusterCreate>;
    fn kubeconfig_update_default(self, kubeconfig_update_default: bool) -> K3d<K3dClusterCreate>;
    fn lb_config_override(self, lb_config_override: String) -> K3d<K3dClusterCreate>;
    fn network(self, network: String) -> K3d<K3dClusterCreate>;
    fn no_image_volume(self, no_image_volume: bool) -> K3d<K3dClusterCreate>;
    fn no_lb(self, no_lb: bool) -> K3d<K3dClusterCreate>;
    fn no_rollback(self, no_rollback: bool) -> K3d<K3dClusterCreate>;
    fn port(self, port: String) -> K3d<K3dClusterCreate>;
    fn registry_config(self, registry_config: String) -> K3d<K3dClusterCreate>;
    fn registry_create(self, registry_create: String) -> K3d<K3dClusterCreate>;
    fn registry_use(self, registry_use: Vec<String>) -> K3d<K3dClusterCreate>;
    fn runtime_label(self, runtime_label: String) -> K3d<K3dClusterCreate>;
    fn runtime_ulimit(self, runtime_ulimit: String) -> K3d<K3dClusterCreate>;
    fn servers(self, servers: u16) -> K3d<K3dClusterCreate>;
    fn servers_memory(self, servers_memory: String) -> K3d<K3dClusterCreate>;
    fn subnet(self, subnet: String) -> K3d<K3dClusterCreate>;
    fn timeout(self, timeout: String) -> K3d<K3dClusterCreate>;
    fn token(self, token: String) -> K3d<K3dClusterCreate>;
    fn volume(self, volume: String) -> K3d<K3dClusterCreate>;
    fn wait(self, wait: bool) -> K3d<K3dClusterCreate>;
}

impl K3dClusterCreateArgs for K3d<K3dClusterCreate> {
    fn agents(mut self, agents: u16) -> K3d<K3dClusterCreate> {
        self.cmd.arg("--agents").arg(agents.to_string());

        self
    }

    fn agents_memory(mut self, agents_memory: String) -> K3d<K3dClusterCreate> {
        self.cmd.arg("--agents-memory").arg(agents_memory);

        self
    }

    fn api_port(mut self, api_port: String) -> K3d<K3dClusterCreate> {
        self.cmd.arg("--api-port").arg(api_port);

        self
    }

    fn config(mut self, config: String) -> K3d<K3dClusterCreate> {
        self.cmd.arg("--config").arg(config);

        self
    }

    fn env(mut self, env: String) -> K3d<K3dClusterCreate> {
        self.cmd.arg("--env").arg(env);

        self
    }

    fn gpus(mut self, gpus: String) -> K3d<K3dClusterCreate> {
        self.cmd.arg("--gpus").arg(gpus);

        self
    }

    fn host_alias(mut self, host_alias: String) -> K3d<K3dClusterCreate> {
        self.cmd.arg("--host-alias").arg(host_alias);

        self
    }

    fn host_pid_mode(mut self, host_pid_mode: bool) -> K3d<K3dClusterCreate> {
        self.cmd
            .arg("--host-pid-mode")
            .arg(host_pid_mode.to_string());

        self
    }

    fn image(mut self, image: String) -> K3d<K3dClusterCreate> {
        self.cmd.arg("--image").arg(image);

        self
    }

    fn k3s_arg(mut self, k3s_arg: String) -> K3d<K3dClusterCreate> {
        self.cmd.arg("--k3s-arg").arg(k3s_arg);

        self
    }

    fn k3s_node_label(mut self, k3s_node_label: String) -> K3d<K3dClusterCreate> {
        self.cmd.arg("--k3s-node-label").arg(k3s_node_label);

        self
    }

    fn kubeconfig_switch_context(
        mut self,
        kubeconfig_switch_context: bool,
    ) -> K3d<K3dClusterCreate> {
        self.cmd
            .arg("--kubeconfig-switch-context")
            .arg(kubeconfig_switch_context.to_string());

        self
    }

    fn kubeconfig_update_default(
        mut self,
        kubeconfig_update_default: bool,
    ) -> K3d<K3dClusterCreate> {
        self.cmd
            .arg("--kubeconfig-update-default")
            .arg(kubeconfig_update_default.to_string());

        self
    }

    fn lb_config_override(mut self, lb_config_override: String) -> K3d<K3dClusterCreate> {
        self.cmd.arg("--lb-config-override").arg(lb_config_override);

        self
    }

    fn network(mut self, network: String) -> K3d<K3dClusterCreate> {
        self.cmd.arg("--network").arg(network);

        self
    }

    fn no_image_volume(mut self, no_image_volume: bool) -> K3d<K3dClusterCreate> {
        self.cmd
            .arg("--no-image-volume")
            .arg(no_image_volume.to_string());

        self
    }

    fn no_lb(mut self, no_lb: bool) -> K3d<K3dClusterCreate> {
        self.cmd.arg("--no-lb").arg(no_lb.to_string());

        self
    }

    fn no_rollback(mut self, no_rollback: bool) -> K3d<K3dClusterCreate> {
        self.cmd.arg("--no-rollback").arg(no_rollback.to_string());

        self
    }

    fn port(mut self, port: String) -> K3d<K3dClusterCreate> {
        self.cmd.arg("--port").arg(port);

        self
    }

    fn registry_config(mut self, registry_config: String) -> K3d<K3dClusterCreate> {
        self.cmd.arg("--registry-config").arg(registry_config);

        self
    }

    fn registry_create(mut self, registry_create: String) -> K3d<K3dClusterCreate> {
        self.cmd.arg("--registry-create").arg(registry_create);

        self
    }

    fn registry_use(mut self, registry_use: Vec<String>) -> K3d<K3dClusterCreate> {
        self.cmd.arg("--registry-use").arg(registry_use.join(","));

        self
    }

    fn runtime_label(mut self, runtime_label: String) -> K3d<K3dClusterCreate> {
        self.cmd.arg("--runtime-label").arg(runtime_label);

        self
    }

    fn runtime_ulimit(mut self, runtime_ulimit: String) -> K3d<K3dClusterCreate> {
        self.cmd.arg("--runtime-ulimit").arg(runtime_ulimit);

        self
    }

    fn servers(mut self, servers: u16) -> K3d<K3dClusterCreate> {
        self.cmd.arg("--servers").arg(servers.to_string());

        self
    }

    fn servers_memory(mut self, servers_memory: String) -> K3d<K3dClusterCreate> {
        self.cmd.arg("--servers-memory").arg(servers_memory);

        self
    }

    fn subnet(mut self, subnet: String) -> K3d<K3dClusterCreate> {
        self.cmd.arg("--subnet").arg(subnet);

        self
    }

    fn timeout(mut self, timeout: String) -> K3d<K3dClusterCreate> {
        self.cmd.arg("--timeout").arg(timeout);

        self
    }

    fn token(mut self, token: String) -> K3d<K3dClusterCreate> {
        self.cmd.arg("--token").arg(token);

        self
    }

    fn volume(mut self, volume: String) -> K3d<K3dClusterCreate> {
        self.cmd.arg("--volume").arg(volume);

        self
    }

    fn wait(mut self, wait: bool) -> K3d<K3dClusterCreate> {
        self.cmd.arg("--wait").arg(wait.to_string());

        self
    }
}

impl<'a> K3dRun<'a> for K3d<K3dClusterCreate> {
    fn run(&'a mut self) -> Result<()> {
        self.cmd.spawn()?.wait_with_output()?;

        Ok(())
    }
}
