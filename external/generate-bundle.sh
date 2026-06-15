#! /bin/bash

set -e # Exit on error

cd "$(dirname "$0")" # cd to this script dir

function bundle () {
	local OUTPUT=$1
	local ASSETS_FILE=$2
	local KEY=$3

	local FOUND=false

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
			if [[ $LINE == *"pub const ${KEY}_ASSETS: [Asset;"* ]]; then
				if [[ $LINE != *"= [asset!("* ]]; then # Check is the not the release array
					FOUND=true # Found start of CSS assets array
					rm -f $OUTPUT
				fi
			fi
		fi
	done < $ASSETS_FILE
}

bundle "../assets/bundle.css" "../src/frontend/assets.rs" "CSS"
bundle "../assets/bundle.js" "../src/frontend/assets.rs" "JS"