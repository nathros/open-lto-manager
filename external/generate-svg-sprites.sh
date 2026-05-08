#! /bin/bash

set -e # Exit on error

function preview_start() {
	OUTPUT=$1
	echo "<!DOCTYPE html>" > $OUTPUT
	echo "<html>" >> $OUTPUT
	echo "	<head>" >> $OUTPUT
	echo "		<title>$2 preview</title>" >> $OUTPUT
	echo "		<style>td, th { border: 1px solid; } td { padding: 4px;} tr th { position: sticky; top: 0; background-color: white; }</style>" >> $OUTPUT
	echo "		<style>img { width: 6rem; height: 6rem; margin: 2px; } .sm { width: 2rem; } .fill { background-color: lightgrey; } a { text-decoration: none; }</style>" >> $OUTPUT
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
	echo "<svg xmlns=\"http://www.w3.org/2000/svg\">" > $OUTPUT
	echo -n "<defs><style>.icon { display: none } .icon:target { display: inline }</style></defs>" >> $OUTPUT
}

function icon_end() {
	OUTPUT=$1
	echo >> $OUTPUT
	echo -n '</svg>' >> $OUTPUT
}

function process_theme() {
	OUTPUT_NAME=$2
	OUTPUT_DIR=$3

	INDEX=0
	declare -A THEME_NAME_INDEX

	THEME_NAME=()
	THEME_PATH=()
	THEME_REPO=()
	THEME_ACTION=()
	THEME_VERSION=()

	SCAN_THEMES=false
	SCAN_ICONS=false

	PREVIEW=${OUTPUT_DIR}${OUTPUT_NAME}.preview.html
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
					icon_start "${OUTPUT_DIR}${OUTPUT_NAME}-${N}.svg"
					echo "	<th>${THEME_NAME[$I]}.svg (${THEME_VERSION[$I]})<a href='${THEME_REPO[$I]}' target='_blank'> &#128279;</a></th>" >> ${PREVIEW}
				done
				
				echo "</tr>" >> ${PREVIEW}

			elif [[ $LINE == *": {" ]]; then
				THEME_NAME+=(${LINE:1:-4}) # Found theme name
				THEME_NAME_INDEX[${LINE:1:-4}]="$INDEX"
				INDEX=$((INDEX + 1))

			elif [[ $LINE == "\"path\": "* ]]; then
				THEME_PATH+=(${LINE:9:-2}) # Found theme path
				cd ../${LINE:9:-2}
				THEME_VERSION+=($(git describe --exact-match --tags 2>/dev/null || true))
				cd - > /dev/null

			elif [[ $LINE == "\"repo\": "* ]]; then
				THEME_REPO+=(${LINE:9:-2}) # Found theme repository

			elif [[ $LINE == "\"action\": "* ]]; then
				THEME_ACTION+=(${LINE:11:-1}) # Found theme viewBox
			fi

		elif $SCAN_ICONS ; then
			if [[ $LINE == "}" ]]; then
				SCAN_ICONS=false # End of icons section

				for N in ${THEME_NAME[*]} ; do # End of .svg
					I=${THEME_NAME_INDEX[$N]}
					icon_end "${OUTPUT_DIR}${OUTPUT_NAME}-${N}.svg"
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
				THEME=`echo "$LINE" | cut -d '"' -f 2`
				ICON_PATH=`echo "$LINE" | cut -d '"' -f 4`
				I=${THEME_NAME_INDEX[$THEME]} # Get icon them index from name
				N=${THEME_NAME[${I}]}
				SVG=$(cat "../${THEME_PATH[$I]}$ICON_PATH")
				FIND="viewBox"
				REPLACE="id=\"$ICON_NAME\" class=\"icon\" $FIND"
				# After: id="achor" class="icon" viewBox
				# First occurrence only

				echo >> "${OUTPUT_DIR}${OUTPUT_NAME}-${N}.svg" # Add new line

				if [[ "${THEME_ACTION[${I}]}" == "tab" ]]; then
					echo "$SVG" | sed -e "s/id\=\"/id\=\"${ICON_NAME}-/g"               `# Append icon name to inner ids to make them unique` \
						| sed -e "s/href=\"#/href=\"#${ICON_NAME}-/g"                   `# Update url(#) with new ids` \
						| sed -e "s/=\"url(#/=\"url(#${ICON_NAME}-/g"                   `# Update href(#) with new ids` \
						| sed -e "s/$FIND/$REPLACE/g"                                   `# Add icon class` \
						| sed -e '1,4d'                                                 `# # tab # Delete lines 1-4 (comments)` \
						| sed -e 's/  \(width\|height\)="[0-9]*"//g'                    `# # tab # Find replace width and height` \
						| sed -r '/^\s*$/d'                                             `# # tab # Remove empty lines` \
						| sed -e 's/<style>/<style>@scope{/g'                           `# Wrap styles inside @scope open` \
						| sed -e 's/<\/style>/}<\/style>/g'                             `# Wrap styles inside @scope close` \
						| sed -e 's/xmlns=\"http:\/\/www.w3.org\/2000\/svg\"//g'        `# Remove xmlns` \
						| tr -d '\n'                                                    `# Remove new lines` \
						| tr -s " " >> "${OUTPUT_DIR}${OUTPUT_NAME}-${N}.svg"           `# Remove whitespace`
				else
					echo "$SVG" | sed -e "s/id\=\"/id\=\"${ICON_NAME}-/g"               `# Append icon name to inner ids to make them unique` \
						| sed -e "s/href=\"#/href=\"#${ICON_NAME}-/g"                   `# Update url(#) with new ids` \
						| sed -e "s/=\"url(#/=\"url(#${ICON_NAME}-/g"                   `# Update href(#) with new ids` \
						| sed -e "s/$FIND/$REPLACE/g"                                   `# Add icon class` \
						| sed -e 's/<style>/<style>@scope{/g'                           `# Wrap styles inside @scope open` \
						| sed -e 's/<\/style>/}<\/style>/g'                             `# Wrap styles inside @scope close` \
						| sed -e 's/xmlns=\"http:\/\/www.w3.org\/2000\/svg\"//g'        `# Remove xmlns` \
						| tr -d '\n'                                                    `# Remove new lines` \
						| tr -s " " >> "${OUTPUT_DIR}${OUTPUT_NAME}-${N}.svg"           `# Remove whitespace`
				fi

				echo "	<td>" >> ${PREVIEW}
				echo "		<img src='./${OUTPUT_NAME}-${N}.svg#$ICON_NAME'>" >> ${PREVIEW}
				echo "		<img class='sm' src='./${OUTPUT_NAME}-${N}.svg#$ICON_NAME'>" >> ${PREVIEW}
				echo "		<img class='fill' src='./${OUTPUT_NAME}-${N}.svg#$ICON_NAME'>" >> ${PREVIEW}
				echo "		<img class='fill sm' src='./${OUTPUT_NAME}-${N}.svg#$ICON_NAME'>" >> ${PREVIEW}
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

git submodule update --progress --init --recursive

#process_theme "icons.json" "icons" "../assets/"
process_theme "logos.json" "logos" "../assets/"
