FROM ghcr.io/catthehacker/ubuntu:act-latest@sha256:117899b2d9cd9876492ebb343f800e5520f3b5f85df8c47bac3fbdbb7f251c9c

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