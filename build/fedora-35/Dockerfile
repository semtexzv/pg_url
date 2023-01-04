FROM fedora:35

ARG USER=docker
ARG UID=1000
ARG GID=1000

RUN useradd -m ${USER} --uid=${UID}
ENV HOME=/home/docker


RUN dnf update -y && dnf install -y git && \
    dnf install -y https://download.postgresql.org/pub/repos/yum/reporpms/F-35-x86_64/pgdg-fedora-repo-latest.noarch.rpm && \
    dnf install -y postgresql11-server postgresql11-devel && \
    dnf install -y postgresql12-server postgresql12-devel &&  \
    dnf install -y postgresql13-server postgresql13-devel &&  \
    dnf install -y postgresql14-server postgresql14-devel && \
    dnf install -y postgresql15-server postgresql15-devel & \
    dnf group install -y "Development Tools" && \
    dnf install -y openssl-devel  redhat-rpm-config ruby-devel gcc make rpm-build libffi-devel && \
    dnf clean all && rm -rf /var/cache/yum && gem install --no-document fpm


USER ${UID}:${GID}
WORKDIR /${HOME}/pkg

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y --profile minimal --default-toolchain stable
ENV PATH="/home/${USER}/.cargo/bin:${PATH}"

RUN cargo install cargo-pgx
RUN cargo pgx init \
	--pg11=/usr/pgsql-11/bin/pg_config \
	--pg12=/usr/pgsql-12/bin/pg_config \
	--pg13=/usr/pgsql-13/bin/pg_config \
	--pg14=/usr/pgsql-14/bin/pg_config \
	--pg15=/usr/pgsql-15/bin/pg_config

USER root:root