FROM ubuntu:kinetic

ARG USER=docker
ARG UID=1000
ARG GID=1000

RUN useradd -m ${USER} --uid=${UID}
ENV HOME=/home/docker

ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y wget gnupg && \
    { echo "deb http://apt.postgresql.org/pub/repos/apt/ kinetic-pgdg main" >> /etc/apt/sources.list.d/pgdg.list; } && \
    { wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | apt-key add - && apt-get update; } && \
    apt-get install -y git curl gcc make build-essential libz-dev zlib1g-dev strace libssl-dev pkg-config && \
    apt-get install -y postgresql-11 postgresql-server-dev-11 && \
    apt-get install -y postgresql-12 postgresql-server-dev-12 && \
    apt-get install -y postgresql-13 postgresql-server-dev-13 && \
    apt-get install -y postgresql-14 postgresql-server-dev-14 && \
    apt-get install -y postgresql-15 postgresql-server-dev-15 && \
    apt-get install -y ruby ruby-dev rubygems build-essential && \
    apt-get clean && gem install --no-document fpm

USER ${UID}:${GID}
WORKDIR ${HOME}/pkg

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y --profile minimal --default-toolchain stable
ENV PATH="${HOME}/.cargo/bin:${PATH}"

RUN cargo install cargo-pgx
RUN cargo pgx init \
	--pg11=/usr/lib/postgresql/11/bin/pg_config \
	--pg12=/usr/lib/postgresql/12/bin/pg_config \
	--pg13=/usr/lib/postgresql/13/bin/pg_config \
	--pg14=/usr/lib/postgresql/14/bin/pg_config \
	--pg15=/usr/lib/postgresql/15/bin/pg_config

USER root:root
