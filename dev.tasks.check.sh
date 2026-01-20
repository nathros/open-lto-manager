#!/bin/bash

GREEN="\e[0;32m"
YELLOW="\e[0;33m"
RESET="\e[0m"

FIXME=$(grep -Hrn 'FIXME' src/)
TODO=$(grep -Hrn 'TODO' src/)

function show() {
	echo -n "Showing: "
	printf %-30s $1 | tr ' ' " "
	echo
	echo ------------------------------------------------------------------------------------------------

	while read -r LINE ; do
    	#echo "Line: $LINE"
		
		printf %-50s $(echo $LINE | cut -d: -f1) | tr ' ' " "
		echo -n " | "
		echo -n "${LINE##*//}"
		echo
	done < <(grep --null -Hrn "$1" src/)
	echo
}

echo -e "${GREEN}This is a list of FIXMEs and TODOs found in the codebase${RESET}"
show "FIXME"
show "TODO"
echo -e "${YELLOW}Idealy these lists should be empty, especially the FIXMEs${RESET}"
