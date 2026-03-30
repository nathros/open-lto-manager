#!/bin/bash

set -e # Stop on error

BASE_DIR=/opt
LTFS_DIR=/opt/ltfs
LTFS_SOURCE=https://github.com/LinearTapeFileSystem/ltfs
BUGGY_IFS=false
GROUP=tape
GROUP_CHANGED=false
USR=$(whoami)

err_msg () {
	echo "This script only supports: Debian(+varients)"
	echo "Options:"
	echo "* Pass OS name as first parameter"
	echo "* Try manual install see: $LTFS_SOURCE"
}

ask_buggy_ifs () {
	echo "*****************************************************************************************"
	echo "* Are you using any of the following controllers:                                       *"
	echo "* ATTO      ExpressSAS H6xx                                                             *"
	echo "* HighPoint RocketRAID 27xx                                                             *"
	echo "* Any USB SAS converter                                                                 *"
	echo "*                                                                                       *"
	echo "* These have a firmware issue which need ltfs configure flag to work around             *"
	echo "* See: $LTFS_SOURCE/wiki/HBA-info                       *"
	echo "*****************************************************************************************"
	while true ; do
		read -p 'y/n: ' answer
		case "${answer}" in
			[yY]|[yY][eE][sS])
			BUGGY_IFS=true
			break
			;;

			[nN]|[nN][oO])
			BUGGY_IFS=false
			break
			;;
		esac
	done
}

check_groups () {
	CURRENT_GROUPS=$(groups $USR)
	if [[ $CURRENT_GROUPS != *"$GROUP"* ]]; then
		sudo usermod -a -G $GROUP $USR
		GROUP_CHANGED=true
	fi
}

install_icu () {
	sudo tee $ICU_PATH >/dev/null <<EOL
#!/bin/sh

opts=\$1

case \$opts in
	'--cppflags')
		echo '' ;;
	'--ldflags')
		echo '-licuuc -licudata' ;;
	*)
	echo '/usr/lib/x86_64-linux-gnu/icu/pkgdata.inc' ;;
esac
EOL

	sudo chmod +x $ICU_PATH
}

install_as_debian () {
	# Adapted from: https://github.com/LinearTapeFileSystem/Debian12-Build

	ask_buggy_ifs

	LTFS="build-essential git pkg-config automake autoconf libtool libfuse-dev fuse uuid-dev libxml2-dev libsnmp-dev libicu-dev icu-devtools"
	MT="mt-st"
	PACKAGES="$LTFS $MT"
	ICU_PATH="/usr/bin/icu-config"

	echo "********************************************************************************************"
	echo "* The following actions will be performed:                                                 *"
	echo "* 1) Packages to be installed: ${PACKAGES:0:56}    *"
	echo "*    ${PACKAGES:57}       *"
	echo "* 2) Add user '$USR' to group '$GROUP'                                                     *"
	echo "* 3) Install icu-config to $ICU_PATH                                             *"
	echo "*    This is deprecated in Debian: $LTFS_SOURCE/issues/153 *"
	echo "* 4) Compile and install LTFS to $LTFS_DIR                                                 *"
	echo "*    LTFS source: $LTFS_SOURCE                             *"
	echo "********************************************************************************************"

	check_groups

	# LTFS
	sudo apt update && apt install -y $PACKAGES

	cd $BASE_DIR
	sudo rm -rf $LTFS_DIR
	sudo git clone $LTFS_SOURCE
	cd $LTFS_DIR
	sudo git checkout $(git describe --tags $(git rev-list --tags --max-count=1))

	install_icu

	sudo ./autogen.sh
	if [ "$BUGGY_IFS" = true ] ; then
		sudo ./configure --enable-buggy-ifs
	else
		sudo ./configure
	fi
	sudo make
	sudo make install
	sudo ldconfig -v
}

# Main

if [ $# -eq 0 ]; then
	echo "No OS specified will try to detect"
else
	OS=$1
	echo "OS $OS has been manually"
fi

if [ -z "$OS" ]; then
	if [ ! -f /etc/os-release ]; then
		echo "File: /etc/os-release not found, unable to determine OS"
		err_msg
		exit 1
	fi
	OS=$(cat /etc/os-release | grep -e "PRETTY_NAME" -e "ID_LIKE")
	OS=${OS//\\n/} # Remove newlines
fi

OS=${OS,,} # Lowercase

if [[ $OS == *"debian"* ]]; then
	echo "Found Debian or (Debian derivative)"
	install_as_debian
else
	echo "Unable to determine supported OS from: $OS"
	err_msg
	exit 1
fi

echo
echo "== INSTALL COMPLETE== "
if [ "$GROUP_CHANGED" = true ] ; then
	echo "Group has been changed, you need to logout and login for this to take effect"
fi
echo "Installed LTFS at: $LTFS_DIR"
echo
echo "Check installed mt version:"
mt --version
echo
echo "Check installed LTFS version:"
ltfs --version
