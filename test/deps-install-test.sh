#!/bin/bash

cd "$(dirname "$0")"
cd ..

# Test deps-install.sh in different environments

MOUNT=$(pwd)/scripts:/scripts
INSTALLER="/scripts/deps-install.sh -u"                             # Unattended install
APT_ARCHIVE="sed -i -e 's/deb\./archive\./g' /etc/apt/sources.list" # Change [deb.debian.org] to [archive.debian.org]

docker run -v $MOUNT -it --rm debian:10 /bin/bash -c "$APT_ARCHIVE && $INSTALLER"
docker run -v $MOUNT -it --rm debian:11 $INSTALLER
docker run -v $MOUNT -it --rm debian:12 $INSTALLER
docker run -v $MOUNT -it --rm debian:13 $INSTALLER

docker run -v $MOUNT -it --rm ubuntu:24.04 $INSTALLER
docker run -v $MOUNT -it --rm ubuntu:26.04 $INSTALLER

docker run -v $MOUNT -it --rm archlinux:base-20260719.0.558177 $INSTALLER

docker run -v $MOUNT -it --rm ghcr.io/void-linux/void-glibc-full:20260701r1 /bin/sh -c "xbps-install -Suy && xbps-install -y bash && $INSTALLER"

#docker run -v $(pwd)/scripts:/scripts --rm -it --entrypoint bash debian:buster
