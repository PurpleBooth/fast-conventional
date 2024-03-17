FROM ghcr.io/catthehacker/ubuntu:act-latest@sha256:f9e2268057283b37d3a10fa9e64a201e3b6c82c84b3704cdd01b0797273d4c3a

# Apt utilis
RUN apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y \
        apt-utils

# Dependencies for JetBrains devcontainer implementation
RUN apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y \
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
RUN apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y \
        zsh \
        fish \
        bash

# Rustup init
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o /usr/local/bin/rustup-init \
    && chmod -v +x /usr/local/bin/rustup-init

# The user
RUN echo '%sudo ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers \
    && addgroup \
      --uid 568 \
      devcontainer \
    && adduser \
      --disabled-password \
      --ingroup sudo \
      --shell "/usr/bin/fish" \
      --gecos '' \
      --uid 568 \
      --ingroup sudo \
      --ingroup devcontainer \
      devcontainer \
    && chown -vR devcontainer:devcontainer \
      /home/devcontainer

USER devcontainer

RUN mkdir -vp /home/devcontainer/.config/fish/conf.d/
RUN rustup-init -y \
      --component rustfmt \
      --component clippy \
      --target aarch64-apple-darwin \
      --target aarch64-unknown-linux-gnu \
      --target x86_64-apple-darwin \
      --target x86_64-pc-windows-gnu \
      --target x86_64-pc-windows-msvc \
      --target x86_64-unknown-linux-gnu


ENV PATH="/home/devcontainers/.cargo/bin:$PATH"
RUN rustup toolchain install stable \
      --allow-downgrade \
      --component clippy \
      --component rustfmt \
      --target aarch64-apple-darwin \
      --target aarch64-unknown-linux-gnu \
      --target i686-pc-windows-gnu \
      --target i686-pc-windows-msvc \
      --target i686-unknown-linux-gnu \
      --target x86_64-apple-darwin \
      --target x86_64-pc-windows-gnu \
      --target x86_64-pc-windows-msvc \
      --target x86_64-unknown-linux-gnu
RUN rustup toolchain install nightly \
      --allow-downgrade \
      --component clippy \
      --component rustfmt \
      --target aarch64-apple-darwin \
      --target aarch64-unknown-linux-gnu \
      --target i686-pc-windows-gnu \
      --target i686-pc-windows-msvc \
      --target i686-unknown-linux-gnu \
      --target x86_64-apple-darwin \
      --target x86_64-pc-windows-gnu \
      --target x86_64-pc-windows-msvc \
      --target x86_64-unknown-linux-gnu