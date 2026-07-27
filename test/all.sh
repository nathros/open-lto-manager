#!/bin/bash

cd "$(dirname "$0")"

if ! ./unit-tests.sh; then
	echo "Failure in unit tests"
	exit 1
fi

if ! ./deps-install-test.sh; then
	echo "Failure in dependencies tests"
	exit 1
fi
