#! /bin/bash

set -e # Exit on error

function preview_start() {
	OUTPUT=$1
	echo "<!DOCTYPE html>" > $OUTPUT
	echo "<html>" >> $OUTPUT
	echo "	<head>" >> $OUTPUT
	echo "		<title>$2 preview</title>" >> $OUTPUT
	echo "		<style>td, th { border: 1px solid; } td { padding: 4px;} table { border-collapse: separate; } tr th { position: sticky; top: 0; background-color: white; }</style>" >> $OUTPUT
	echo "		<style>img { width: 6rem; } .sm { width: 2rem; } .fill { background-color: lightgrey } a { text-decoration: none; }</style>" >> $OUTPUT
	echo "	</head>" >> $OUTPUT
	echo "<body>" >> $OUTPUT
	echo "<p>This is a preview of SVG sprites which are accessed via: #anchor</p>" >> $OUTPUT
	echo >> $OUTPUT
	echo "<table>" >> $OUTPUT
}

function preview_end() {
	OUTPUT=$1
	echo "</tr>" >> $OUTPUT
	echo "</table>" >> $OUTPUT
	echo >> $OUTPUT
	echo "</body>" >> $OUTPUT
	echo "</html>" >> $OUTPUT
}

function icon_start() {
	OUTPUT=$1
	VIEW=$2
	echo "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 ${VIEW} ${VIEW}\">" > $OUTPUT
	echo "<defs><style>svg .icon { display: none } svg .icon:target { display: inline }</style></defs>" >> $OUTPUT
}

function icon_end() {
	OUTPUT=$1
	echo '</svg>' >> $OUTPUT
}

function process_theme() {
	OUTPUT_NAME=$2

	INDEX=0
	declare -A THEME_NAME_INDEX

	THEME_NAME=()
	THEME_PATH=()
	THEME_REPO=()
	THEME_VIEWBOX=()
	THEME_VERSION=()

	SCAN_THEMES=false
	SCAN_ICONS=false

	PREVIEW=${OUTPUT_NAME}.preview.html
	preview_start ${PREVIEW} ${OUTPUT_NAME}

	while read -r LINE; do
		#echo $LINE
		if $SCAN_THEMES ; then
			if [[ $LINE == "}" ]]; then
				SCAN_THEMES=false # End of themes section
				
				echo "<tr>" >> ${PREVIEW}
				echo "	<th>Icon Anchor</th>" >> ${PREVIEW}

				for N in ${THEME_NAME[*]} ; do # Start of .svg
					I=${THEME_NAME_INDEX[$N]}
					icon_start "${N}.svg" "${THEME_VIEWBOX[$I]}"
					echo "	<th>${THEME_NAME[$I]}.svg (${THEME_VERSION[$I]})<a href='${THEME_REPO[$I]}' target='_blank'> &#128279;</a></th>" >> ${PREVIEW}
				done
				
				echo "</tr>" >> ${PREVIEW}

			elif [[ $LINE == *": {" ]]; then
				THEME_NAME+=(${LINE:1:-4}) # Found theme name
				THEME_NAME_INDEX[${LINE:1:-4}]="$INDEX"
				INDEX=$((INDEX + 1))

			elif [[ $LINE == "\"path\": "* ]]; then
				THEME_PATH+=(${LINE:9:-2}) # Found theme path
				cd ../../${LINE:9:-2}
				THEME_VERSION+=($(git describe --exact-match --tags))
				cd - > /dev/null

			elif [[ $LINE == "\"repo\": "* ]]; then
				THEME_REPO+=(${LINE:9:-2}) # Found theme repository

			elif [[ $LINE == "\"viewbox\": "* ]]; then
				THEME_VIEWBOX+=(${LINE:12:-1}) # Found theme viewBox
			fi

		elif $SCAN_ICONS ; then
			if [[ $LINE == "}" ]]; then
				SCAN_ICONS=false # End of icons section

				for N in ${THEME_NAME[*]} ; do # End of .svg
					I=${THEME_NAME_INDEX[$N]}
					icon_end "${N}.svg" "${THEME_VIEWBOX[$I]}"
				done

			elif [[ $LINE == *": {" ]]; then
				ICON_NAME=${LINE:1:-4}

				echo "<tr>" >> ${PREVIEW}
				echo "	<td>$ICON_NAME</td>" >> ${PREVIEW}

			elif [[ $LINE == *"},"* ]]; then
				echo "</tr>" >> ${PREVIEW}
				continue

			else
				# Add id and class to inner svg
				THEME=`echo "$LINE" | cut -d'"' -f 2`
				ICON_PATH=`echo "$LINE" | cut -d'"' -f 4`
				I=${THEME_NAME_INDEX[$THEME]} # Get icon them index from name
				N=${THEME_NAME[${I}]}
				SVG=$(cat "../../${THEME_PATH[$I]}$ICON_PATH")
				FIND="viewBox"
				REPLACE="id=\"$ICON_NAME\" class=\"icon\" $FIND"
				# After: id="achor" class="icon" viewBox
				# First occurrence only
				echo "${SVG/$FIND/$REPLACE}" >> "${N}.svg"

				echo "	<td>" >> ${PREVIEW}
				echo "		<img src='./${N}.svg#$ICON_NAME'>" >> ${PREVIEW}
				echo "		<img class='sm' src='./${N}.svg#$ICON_NAME'>" >> ${PREVIEW}
				echo "		<img class='fill' src='./${N}.svg#$ICON_NAME'>" >> ${PREVIEW}
				echo "		<img class='fill sm' src='./${N}.svg#$ICON_NAME'>" >> ${PREVIEW}
				echo "	</td>" >> ${PREVIEW}
			fi

		else
			if [[ $LINE == "\"themes\": {" ]]; then
				SCAN_THEMES=true
				SCAN_ICONS=false
			elif [[ $LINE == "\"icons\": {" ]]; then
				SCAN_ICONS=true
				SCAN_THEMES=false
			fi
		fi
	done < "$1"

	preview_end ${PREVIEW}
}

cd "$(dirname "$0")" # cd to this script dir

#git submodule update --progress --init --recursive

process_theme "icons.json" "icons"
