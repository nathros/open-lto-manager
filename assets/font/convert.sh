#!/bin/bash

set -e # Stop on error

cd "$(dirname "$0")"

for FILE in *;
do 
	if [[ $FILE = *".woff2" ]]; then
		woff2_decompress $FILE
	fi
done
