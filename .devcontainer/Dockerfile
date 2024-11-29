FROM ghcr.io/catthehacker/ubuntu:act-latest@sha256:9718faaa7294a970af4f9a9f301c56e67fe5b538a6be9fac37286e2700648691

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