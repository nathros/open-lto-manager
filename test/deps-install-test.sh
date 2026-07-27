#!/bin/bash

cd "$(dirname "$0")"
cd ..

# Test deps-install.sh in different environments

MOUNT=$(pwd)/scripts:/scripts
INSTALLER="/scripts/deps-install.sh -u"                             # Unattended install
APT_ARCHIVE="sed -i -e 's/deb\./archive\./g' /etc/apt/sources.list" # Change [deb.debian.org] to [archive.debian.org]

docker run -v $MOUNT -it --rm debian:buster /bin/bash -c "$APT_ARCHIVE && $INSTALLER"
docker run -v $MOUNT -it --rm debian:bullseye $INSTALLER
docker run -v $MOUNT -it --rm debian:bookworm $INSTALLER
docker run -v $MOUNT -it --rm debian:trixie $INSTALLER

docker run -v $MOUNT -it --rm ubuntu:24.04 $INSTALLER
docker run -v $MOUNT -it --rm ubuntu:26.04 $INSTALLER

#docker run -v $(pwd)/scripts:/scripts --rm -it --entrypoint bash debian:buster
