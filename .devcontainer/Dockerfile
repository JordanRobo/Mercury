FROM mcr.microsoft.com/devcontainers/rust:1-bullseye

# Install SQLite and other dependencies
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends sqlite3 libsqlite3-dev pkg-config \
    && apt-get clean -y && rm -rf /var/lib/apt/lists/*

# Install Diesel CLI with SQLite features
RUN cargo install diesel_cli --no-default-features --features sqlite

# Install Bun
RUN curl -fsSL https://bun.sh/install | bash

# Ensure user has permissions to workspace
RUN mkdir -p /workspace && chown -R $USER:$USER /workspace