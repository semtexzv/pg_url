#!/usr/bin/env bash

set -exuo

OSNAME=$1
#OSNAME=$(cat /etc/*-release | grep -oE "(DISTRIB_ID=)(.*)" | awk -F= '{print $2}')
VERSION=$(perl -ne 'print "$1\n" if /version\s*=\s*"(.*)"/' Cargo.toml | head -1)

ARCH=$(uname -m)
ARTIFACTDIR=/artifacts
OUTDIR=/home/docker/pkg/target/release/pg_url-pg${PG_VERSION}

PG_CONFIG_DIR=$(dirname $(grep ${PG_VERSION} ~/.pgx/config.toml | cut -f2 -d= | cut -f2 -d\"))
export PATH=${PG_CONFIG_DIR}:${PATH}

rustup update || exit 1
cargo pgx package || exit $?

cd $OUTDIR && fpm \
  -s dir \
  -t deb \
  -n pg_url-${PG_VERSION} \
  -v ${VERSION} \
  --deb-no-default-config-files \
  -p ${ARTIFACTDIR}/pg_url_${OSNAME}_pg${PG_VERSION}-${VERSION}_$ARCH.deb \
  -a $ARCH \
  . || exit 1
