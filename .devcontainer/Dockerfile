FROM ghcr.io/catthehacker/ubuntu:act-latest@sha256:f9e2268057283b37d3a10fa9e64a201e3b6c82c84b3704cdd01b0797273d4c3a

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