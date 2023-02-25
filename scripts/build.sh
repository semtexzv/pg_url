#!/usr/bin/env bash

set -exu

CONTAINER=${1:?Missing container name}
ARCH=${2:?Missing architecture}
VER=${3:?Missing Version}

mkdir -p artifacts

docker buildx build --platform="linux/$ARCH" -f build/${CONTAINER}/Dockerfile -t "pgx-${CONTAINER}-$ARCH" .

docker run \
  -v "$PWD":/home/docker/pkg \
  -v "$PWD/artifacts":/artifacts \
  -e PG_VERSION="$VER" \
  "pgx-${CONTAINER}-$ARCH" ./scripts/package.sh $CONTAINER
