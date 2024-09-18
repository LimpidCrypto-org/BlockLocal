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

##### Frontend dependencies #####

# Install Node.js and npm
RUN curl -fsSL https://deb.nodesource.com/setup_lts.x | bash - \
    && apt-get install -y nodejs

# Install Angular CLI
RUN npm install -g @angular/cli
