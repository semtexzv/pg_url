FROM fedora:37

ARG USER=docker
ARG UID=1000
ARG GID=1000

RUN useradd -m ${USER} --uid=${UID}
ENV HOME=/home/docker

RUN dnf update -y
RUN dnf install -y git
RUN dnf install -y https://download.postgresql.org/pub/repos/yum/reporpms/F-37-x86_64/pgdg-fedora-repo-latest.noarch.rpm
RUN dnf install -y postgresql11-server postgresql11-devel
RUN dnf install -y postgresql12-server postgresql12-devel
RUN dnf install -y postgresql13-server postgresql13-devel
RUN dnf install -y postgresql14-server postgresql14-devel
RUN dnf install -y postgresql15-server postgresql15-devel
RUN dnf group install -y "Development Tools"
RUN dnf install -y openssl-devel
RUN dnf install -y redhat-rpm-config
RUN dnf install -y ruby-devel gcc make rpm-build libffi-devel
RUN dnf update -y
RUN gem install --no-document fpm


USER ${UID}:${GID}
WORKDIR /home/${USER}

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
ENV PATH="/home/${USER}/.cargo/bin:${PATH}"

RUN cargo install cargo-pgx
RUN cargo pgx init \
	--pg11=/usr/pgsql-11/bin/pg_config \
	--pg12=/usr/pgsql-12/bin/pg_config \
	--pg13=/usr/pgsql-13/bin/pg_config \
	--pg14=/usr/pgsql-14/bin/pg_config \
	--pg15=/usr/pgsql-15/bin/pg_config

USER root:root
