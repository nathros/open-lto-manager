#! /bin/bash

set -e # Exit on error

cd "$(dirname "$0")" # cd to this script dir

OUTPUT=../assets/bundle.css
ASSETS_FILE=../src/frontend/assets.rs

FOUND=false

while IFS= read -r LINE; do
	#echo "$LINE"

	if $FOUND ; then
		if [[ $LINE == *"];"* ]]; then
			break
		else
			CUT=`echo "$LINE" | cut -d '"' -f 2`
			cat "..$CUT" >> $OUTPUT
		fi
	else
		if [[ $LINE == *"pub const CSS_ASSETS: [Asset;"* ]]; then
			if [[ $LINE != *"= [asset!("* ]]; then # Check is the not the release array
				FOUND=true # Found start of CSS assets array
				rm -f $OUTPUT
			fi
		fi
	fi
done < $ASSETS_FILE