#!/bin/bash

set -e # Stop on error

BASE_DIR=/opt
LTFS_DIR=/opt/ltfs
LTFS_SOURCE=https://github.com/LinearTapeFileSystem/ltfs
BUGGY_IFS=false
GROUP=tape
GROUP_CHANGED=false
ICU_PATH="/usr/bin/icu-config"
UNATTENDED=false
SUPPORTED="Debian+(derivatives,Ubuntu), Arch, Fedora, RHEL, Rocky, CentOS Stream, OpenSUSE and Void"
IN_DOCKER=true
if [ ! -f /.dockerenv ]; then
	IN_DOCKER=false
fi
if [ -z "$SUDO_USER" ]; then
	USR=$(whoami)
else
	USR=$SUDO_USER
fi

y_n_exit () {
	if [ "$UNATTENDED" = true ]; then
		return 0
	fi
	while true ; do
		read -p 'y/n: ' answer
		case "${answer}" in
			[yY]|[yY][eE][sS])
			break
			;;

			[nN]|[nN][oO])
			exit 0
			break
			;;
		esac
	done
}

do_msg () {
	echo "********************************************************************************************"
	echo "The following actions will be performed:"
	echo " 1) Packages to be installed: $PACKAGES"
	echo " 2) Add user '$USR' to group '$GROUP'"
	echo " 3) Compile and install LTFS to $LTFS_DIR"
	echo "    LTFS source: $LTFS_SOURCE"
	echo "********************************************************************************************"
	echo "Continue?"
	y_n_exit
}

do_msg_debian () {
	echo "********************************************************************************************"
	echo "The following actions will be performed:"
	echo " 1) Packages to be installed: $PACKAGES"
	echo " 2) Add user '$USR' to group '$GROUP'"
	echo " 3) Install icu-config to $ICU_PATH"
	echo "    This is deprecated in Debian: $LTFS_SOURCE/issues/153"
	echo " 4) Compile and install LTFS to $LTFS_DIR"
	echo "    LTFS source: $LTFS_SOURCE"
	echo "********************************************************************************************"
	echo "Continue?"
	y_n_exit
}

err_msg () {
	echo "This script only supports: $SUPPORTED"
	echo "Option 1"
	echo "	Run this script with one of the supported OSs above: $(basename "$0") -o {option}"
	echo "	Or if all required packages are already installed"
	echo "	Run this script with: $(basename "$0") -o generic"
	echo "Option 2"
	echo "	Try manual install see: $LTFS_SOURCE"
}

ask_buggy_ifs () {
	if [ "$UNATTENDED" = true ]; then
		return 0
	fi
	echo "*****************************************************************************************"
	echo "Are you using any of the following controllers:"
	echo "ATTO       ExpressSAS H6xx"
	echo "HighPoint  RocketRAID 27xx"
	echo "Any        USB SAS converter"
	echo ""
	echo "These have a firmware issue which need LTFS configure --enable-buggy-ifs flag to work"
	echo "See: $LTFS_SOURCE/wiki/HBA-info"
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
	if [[ $(cat /etc/group | grep "${GROUP}:") != *"$GROUP"* ]]; then
		echo "Failed to find system group: $GROUP"
	else
		CURRENT_GROUPS=$(groups $USR)
		if [[ $CURRENT_GROUPS != *"$GROUP"* ]]; then
			usermod -a -G $GROUP $USR
			GROUP_CHANGED=true
		fi
	fi
}

install_icu () {
	tee $ICU_PATH >/dev/null <<EOL
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

	chmod +x $ICU_PATH
}

checkout_ltfs () {
	cd $BASE_DIR
	rm -rf $LTFS_DIR
	git clone $LTFS_SOURCE
	cd $LTFS_DIR
	git submodule update --init --recursive
	git checkout $(git describe --tags $(git rev-list --tags --max-count=1)) # Get latest tagged version
}

build_ltfs () {
	if [[ $1 == *"flag"* ]]; then
		# https://github.com/LinearTapeFileSystem/ltfs/issues/571#issuecomment-4046698663
		export CFLAGS="$CFLAGS -Wno-error=declaration-after-statement"
	fi

	./autogen.sh
	if [ "$BUGGY_IFS" = true ] ; then
		./configure --enable-buggy-ifs
	else
		./configure
	fi

	if [[ $1 == *"rhel"* ]]; then
		# https://github.com/LinearTapeFileSystem/ltfs/issues/394#issuecomment-2082624342
		#sed -i 's/,-Wp/ -Wp/g' src/libltfs/Makefile ./conf/Makefile ./init.d/Makefile ./man/Makefile ./messages/Makefile ./src/iosched/Makefile ./src/kmi/Makefile ./src/libltfs/Makefile ./src/tape_drivers/freebsd/cam/Makefile ./src/tape_drivers/generic/file/Makefile ./src/tape_drivers/generic/itdtimg/Makefile ./src/tape_drivers/linux/lin_tape/Makefile ./src/tape_drivers/linux/sg/Makefile ./src/tape_drivers/netbsd/scsipi-ibmtape/Makefile ./src/tape_drivers/osx/iokit/Makefile ./src/utils/Makefile ./src/Makefile ./Makefile
		find . -type f -name 'Makefile' -exec sed -i 's/,-Wp/ -Wp/g' {} \;
	fi

	make -j$(nproc)
	make install -j$(nproc)
	ldconfig -v
}

remove_ltfs () {
	if [ -d $LTFS_DIR ]; then
		cd $LTFS_DIR
		make uninstall
		cd --
		rm -rf $LTFS_DIR
		echo "Uninstalled LTFS from $LTFS_DIR"
	else
		echo "Unable to find LTFS dir: $LTFS_DIR"
		exit 1
	fi
}

install_as_debian () {
	# Adapted from: https://github.com/LinearTapeFileSystem/Debian12-Build
	LTFS="build-essential git pkg-config automake autoconf libtool libfuse-dev fuse uuid-dev libxml2-dev libsnmp-dev libicu-dev icu-devtools"
	MT="mt-st"
	PACKAGES="$LTFS $MT"

	do_msg_debian

	if [ "$IN_DOCKER" = true ]; then
		export DEBIAN_FRONTEND=noninteractive
	fi

	ask_buggy_ifs
	apt update && apt install -y $PACKAGES
	check_groups
	checkout_ltfs
	install_icu
	build_ltfs
}

install_as_arch () {
	PACKAGES="base-devel git make automake autoconf libtool fuse net-snmp"
	do_msg
	ask_buggy_ifs
	pacman -Sy
	pacman -Syu --noconfirm $PACKAGES

	check_groups
	checkout_ltfs
	build_ltfs "flag"
}

install_as_void () {
	# https://github.com/void-linux/void-packages/pull/50845/changes
	PACKAGES="base-devel git make automake autoconf libtool pkg-config icu fuse-devel libuuid-devel libxml2-devel icu icu-devel net-snmp-devel pciutils-devel pcre-devel libsensors-devel libnl3-devel python3-pyxattr"

	do_msg
	ask_buggy_ifs
	xbps-install -Suy
	xbps-install -y $PACKAGES

	check_groups
	checkout_ltfs
	build_ltfs "flag"
}

install_as_suse () {
	PACKAGES="gcc git make automake autoconf libtool icu fuse-devel libuuid-devel libxml2-devel icu libicu-devel net-snmp-devel pciutils-devel pcre-devel libnl3-devel"

	do_msg
	ask_buggy_ifs
	zypper update -y
	zypper install -y $PACKAGES

	check_groups
	checkout_ltfs
	build_ltfs
}

install_as_rhel () {
	PACKAGES="git automake autoconf libtool make icu libicu-devel libxml2-devel libuuid-devel fuse-devel net-snmp-devel python3"
	do_msg
	ask_buggy_ifs

	local NAME=$(cat /etc/os-release | grep PRETTY | tr '[:upper:]' '[:lower:]')
	if [[ $NAME != *"fedora"* && $NAME != *"rocky"* ]]; then
		dnf config-manager --set-enabled crb
	fi
	if [[ $NAME == *"rocky"* ]]; then
		yum -y install dnf-plugins-core
		dnf config-manager --set-enabled crb
	fi

	yum -y update
	yum -y install $PACKAGES

	check_groups
	checkout_ltfs
	build_ltfs "rhel flag"
}

install_as_generic () {
	ask_buggy_ifs
	check_groups
	checkout_ltfs
	build_ltfs "flag"
}

# Main
while getopts 'o:p:ruh' OPT; do
	case "$OPT" in
		o)
			OS="$OPTARG"
			;;
		p)
			BASE_DIR="$OPTARG"
			LTFS_DIR="$BASE_DIR/ltfs"
			;;
		u)
			UNATTENDED=true
			;;
		r)
			REMOVE=true
			;;
		?|h)
			echo "Usage: $(basename $0) [-h] [-u] [-o os-name]"
			echo "[-h]            This menu"
			echo "[-u]            Unattended install"
			echo "[-r]            Remove LTFS"
			echo "[-p path]       LTFS download and build path, default: $LTFS_DIR"
			echo "[-o os-name]    Set script operating system name eg: debian"
			echo "                Each OS has subtle different environment differences"
			echo "                Supported: $SUPPORTED"
			exit 1
			;;
	esac
done
shift "$(($OPTIND -1))"

if [ "$REMOVE" = true ] ; then
	remove_ltfs
	exit 0
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

if [[ $OS == *"debian"* || $OS == *"ubuntu"* ]]; then
	echo "Process as Debian or (Debian derivative)"
	install_as_debian
elif [[ $OS == *"arch"* ]]; then
	echo "Process as Arch or (Arch derivative)"
	GROUP=storage
	install_as_arch
elif [[ $OS == *"void"* ]]; then
	echo "Process as Void Linux"
	install_as_void
elif [[ $OS == *"suse"* ]]; then
	echo "Process as openSUSE"
	install_as_suse
elif [[ $OS == *"fedora"* || $OS == *"centos"* || $OS == *"red hat"* || $OS == *"rhel"* ]]; then
	echo "Process as RHEL or derivative"
	install_as_rhel
elif [[ $OS == *"generic"* ]]; then
	install_as_generic
else
	echo "Unable to determine supported OS from: $OS"
	err_msg
	exit 1
fi

echo
echo "=========================="
echo "==== INSTALL COMPLETE ===="
echo "=========================="
if [ "$GROUP_CHANGED" = true ] ; then
	echo "Group has been changed for user: $USR, logout and login for this to take effect"
fi
echo "Installed LTFS to: $LTFS_DIR"
echo
#echo "Check installed mt version:"
#mt --version
#echo
echo "Check installed LTFS version:"
ltfs --version
