#!/bin/bash

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# Add cargo to PATH
source $HOME/.cargo/env

# Install k3s
curl -sfL https://get.k3s.io | sh -s - --disable-agent --write-kubeconfig-mode 644

