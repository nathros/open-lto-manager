#!/bin/bash

GREEN="\e[0;32m"
YELLOW="\e[0;33m"
RED="\e[0;31m"
RESET="\e[0m"

MIN_RUST_VER="1.81.0"
MIN_DIOXUS_VER="0.7.0"

FAILURE=false

check_ver () {
    if [ "$(printf '%s\n' "$3" "$4" | sort -V | head -n1)" = "$3" ]; then
        echo -e "$1    ($2) :$GREEN FOUND $(printf %-8s $4 | tr ' ' ' ')$RESET minimum[$3]"
    else
        echo -e "$1    ($2) :$YELLOW FOUND $(printf %-8s $4 | tr ' ' ' ')$RESET minimum[$3]"
        FAILURE=true
    fi
}

echo "Run through development environment check list:"

RUST_COMPILER="rustc"
if ! command -v $RUST_COMPILER >/dev/null 2>&1; then
    echo -e "Rust compiler    ($RUST_COMPILER) :$RED NOT FOUND$RESET $(printf %-4s $4 | tr ' ' ' ')$RESET minimum[$MIN_RUST_VER]"
    FAILURE=true
else
    VER=$($RUST_COMPILER --version | awk '{print $2}')
    check_ver "Rust compiler" $RUST_COMPILER $MIN_RUST_VER $VER
fi

DIOXUS="dx"
if ! command -v $DIOXUS >/dev/null 2>&1; then
    echo -e "Dioxus toolchain ($DIOXUS)    :$RED NOT FOUND$RESET $(printf %-4s $4 | tr ' ' ' ')$RESET minimum[$MIN_DIOXUS_VER]"
    FAILURE=true
else
    VER=$($DIOXUS --version | awk '{print $2}')
    check_ver "Dioxus toolchain" $DIOXUS $MIN_DIOXUS_VER $VER
fi

if $FAILURE; then
    echo "Faults found"
    echo "See: https://dioxuslabs.com/learn/0.7/getting_started/"
else
    echo "No faults found"
fi
