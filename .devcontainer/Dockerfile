FROM ghcr.io/catthehacker/ubuntu:act-latest@sha256:f40996550fe09f7c3406a4c5d8ecafc758d336b42342e9653c009ee61ea0a73e

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