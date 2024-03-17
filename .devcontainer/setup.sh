#!/usr/bin/env bash

set -euo pipefail

# Apt utilis
sudo apt-get update \
    && DEBIAN_FRONTEND=noninteractive sudo apt-get install -y \
        apt-utils

# Dependencies for JetBrains devcontainer implementation
sudo apt-get update \
    && DEBIAN_FRONTEND=noninteractive sudo apt-get install -y \
        curl \
        unzip \
        procps \
        libxext6 \
        libxrender1  \
        libxtst6 \
        libxi6 \
        libfreetype6 \
        procps

# Shell
sudo apt-get update \
    && DEBIAN_FRONTEND=noninteractive sudo apt-get install -y \
        zsh \
        fish \
        bash

# Rustup init
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o /usr/local/bin/rustup-init \
    && chmod -v +x /usr/local/bin/rustup-init
