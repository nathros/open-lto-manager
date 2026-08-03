#!/bin/bash

set -e
cd "$(dirname "$0")"
cd ..

# Test scripts/deps-install.sh in different environments

MOUNT=$(pwd)/scripts:/scripts
INSTALLER="/scripts/deps-install.sh -u"                             # Unattended install for LTFS + other dependencies
REMOVE="/scripts/deps-install.sh -r"                                # Uninstall
APT_ARCHIVE="sed -i -e 's/deb\./archive\./g' /etc/apt/sources.list" # Change [deb.debian.org] to [archive.debian.org]

docker run -v $MOUNT -it --rm debian:10 /bin/bash -c "$APT_ARCHIVE && $INSTALLER && $REMOVE"
docker run -v $MOUNT -it --rm debian:11 /bin/bash -c "$INSTALLER && $REMOVE"
docker run -v $MOUNT -it --rm debian:12 /bin/bash -c "$INSTALLER && $REMOVE"
docker run -v $MOUNT -it --rm debian:13 /bin/bash -c "$INSTALLER && $REMOVE"

docker run -v $MOUNT -it --rm ubuntu:24.04 /bin/bash -c "$INSTALLER && $REMOVE"
docker run -v $MOUNT -it --rm ubuntu:26.04 /bin/bash -c "$INSTALLER && $REMOVE"

docker run -v $MOUNT -it --rm archlinux:base-20260719.0.558177 /bin/bash -c "$INSTALLER && $REMOVE"

docker run -v $MOUNT -it --rm ghcr.io/void-linux/void-glibc-full:20260701r1 /bin/sh -c "xbps-install -Suy && xbps-install -y bash && /bin/bash -c \"$INSTALLER && $REMOVE\""

docker run -v $MOUNT -it --rm dokken/centos-stream-9:latest /bin/bash -c "$INSTALLER && $REMOVE"
docker run -v $MOUNT -it --rm dokken/centos-stream-10:latest /bin/bash -c "$INSTALLER && $REMOVE"
docker run -v $MOUNT -it --rm fedora:44 /bin/bash -c "$INSTALLER && $REMOVE"
docker run -v $MOUNT -it --rm rockylinux/rockylinux:9-ubi-init /bin/bash -c "$INSTALLER && $REMOVE"
docker run -v $MOUNT -it --rm rockylinux/rockylinux:10 /bin/bash -c "$INSTALLER && $REMOVE"

docker run -v $MOUNT -it --rm opensuse/leap /bin/bash -c "$INSTALLER && $REMOVE"

#docker run -v $MOUNT -it --rm aclemons/slackware:15.0 /bin/bash -c "$INSTALLER && $REMOVE"

#docker run -v $(pwd)/scripts:/scripts --rm -it --entrypoint bash aclemons/slackware:15.0
