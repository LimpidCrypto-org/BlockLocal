FROM mcr.microsoft.com/devcontainers/rust:1-1-bookworm

##### Backend dependencies #####
RUN apt-get update \
    && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    libssl3 \
    postgresql-client

# Install sea-orm-cli
RUN cargo install sea-orm-cli

# # install k3d
# RUN curl -s https://raw.githubusercontent.com/k3d-io/k3d/main/install.sh | bash

# # install kubectl
# RUN curl -LO https://storage.googleapis.com/kubernetes-release/release/$(curl -s https://storage.googleapis.com/kubernetes-release/release/stable.txt)/bin/linux/amd64/kubectl
# RUN chmod +x ./kubectl
# RUN sudo mv ./kubectl /usr/local/bin/kubectl