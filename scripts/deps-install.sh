#!/bin/bash

set -e # Stop on error

LTFS_VERSION=v2.4.8.1-10519
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
	echo "* Try manual install see: https://github.com/LinearTapeFileSystem/ltfs"
}

get_root () {
	if sudo su ; then
		echo ""
	else
		exit 0
	fi
}

ask_buggy_ifs () {
	echo "*****************************************************************************************"
	echo "* Are you using any of the following controllers:                                       *"
	echo "* ATTO      ExpressSAS H6xx                                                             *"
	echo "* HighPoint RocketRAID 27xx                                                             *"
	echo "* Any USB SAS converter                                                                 *"
	echo "*                                                                                       *"
	echo "* These have a firmware issue which need ltfs configure flag to work around             *"
	echo "* See: https://github.com/LinearTapeFileSystem/ltfs/wiki/HBA-info                       *"
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
		usermod -a -G $GROUP $USR
		GROUP_CHANGED=true
	fi
}

install_as_debian () {
	# Taken from: https://github.com/LinearTapeFileSystem/Debian12-Build

	ask_buggy_ifs

	LTFS="build-essential git pkg-config automake autoconf libtool libfuse-dev fuse uuid-dev libxml2-dev libsnmp-dev libicu-dev icu-devtools"
	MT="cpio"
	PACKAGES="$LTFS $MT"
	ICU_PATH="/usr/bin/icu-config"

	echo "********************************************************************************************"
	echo "* The following actions will be performed:                                                 *"
	echo "* 1) Packages to be installed: ${PACKAGES:0:56}    *"
	echo "*    ${PACKAGES:57}        *"
	echo "* 2) Add user '$USR' to group '$GROUP'                                                     *"
	echo "* 3) Compile and install LTFS to $LTFS_DIR                                                 *"
	echo "     LTFS source: $LTFS_SOURCE                             *"
	echo "* 4) Install icu-config to $ICU_PATH                                             *"
	echo "*    This is deprecated in Debian: https://github.com/LinearTapeFileSystem/ltfs/issues/153 *"
	echo "********************************************************************************************"

	get_root
	check_groups

	# LTFS
	apt update && apt install -y $PACKAGES

	cd $BASE_DIR
	rm -rf $LTFS_DIR
	git clone $LTFS_SOURCE
	cd $LTFS_DIR
	git checkout $LTFS_VERSION
	

	cat >$ICU_PATH <<EOL
#!/bin/sh

opts=$1

case $opts in
	'--cppflags')
		echo '' ;;
	'--ldflags')
		echo '-licuuc -licudata' ;;
	*)
	echo '/usr/lib/x86_64-linux-gnu/icu/pkgdata.inc' ;;
esac
EOL

	chmod +x $ICU_PATH

	./autogen.sh
	if [ "$BUGGY_IFS" = true ] ; then
		./configure --enable-buggy-ifs
	else
		./configure
	fi
	make
	make install
	ldconfig -v
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

if [ "$GROUP_CHANGED" = true ] ; then
	echo "Group has been changed, you need to logout and login for this to take effect"
else
echo "Installed LTFS at: $LTFS_DIR"
echo
echo "Check installed mt version:"
mt --version
echo
echo "Check installed LTFS version:"
ltfs --version
