<div align="center">
  <img src="https://raw.githubusercontent.com/nathros/open-lto-manager/main/assets/logo.svg" alt="Logo" width="128">

  <h2 align="center">Open LTO Manager</h2>

  <p align="center">
    Simple manager for LTO tapes
    <br />
    <!--<a href="https://github.com/nathros/lto-manager"><strong>Explore the docs »</strong></a>-->
    <!--<br />-->
    <!--<br />-->
    <!--<a href="https://github.com/nathros/lto-manager">View Demo</a>-->
    <a href="https://github.com/nathros/open-lto-manager/issues">Report Bug</a> ·
    <a href="https://github.com/nathros/open-lto-manager/issues">Request Feature</a>
  </p>
</div>

## Dependencies
For LTFS support you will need LTO-5 or newer and have a LTFS driver installed, [OpenLTFS](https://github.com/LinearTapeFileSystem/ltfs) is recommended, others are [HPE LTFS](https://support.hpe.com/connect/s/softwaredetails?language=en_US&collectionId=MTX-882b042a6fc04042&tab=releaseNotes) or [IBM LTFS](https://www.ibm.com/support/fixcentral/swg/selectFixes?parent=Tape%20drivers%20and%20software&product=ibm/Storage_Tape/Long+Term+File+System+LTFS&release=2.4&platform=Linux&function=all).
Tar support will need `mt` installed. For most distributions access to tape devices `/dev/nst[x]` and `/dev/st[x]` the user will need to be part of the `tape` group (for Arch this is `storage`).

## Install
### Recommended dependencies
```shell
sudo bash <(curl -L https://raw.githubusercontent.com/nathros/open-lto-manager/main/scripts/deps-install.sh)
```
##### This compiles and installs OpenLTFS, supported OS: Debian, Ubuntu, Arch, Fedora, RHEL, Rocky, CentOS Stream, OpenSUSE and Void +(derivatives of these).
<!--
### Recommended dependencies
```shell
sudo bash <(curl -L https://raw.githubusercontent.com/nathros/open-lto-manager/main/scripts/release-install.sh)
```
-->
<!--### Manual install-->

## Config
### Environment variables
| Variable       | Default             | Description                                               |
| -------------- | ------------------- | --------------------------------------------------------- |
| PATH_DATA      | data                | Application data path which include logs and the database |
| PATH_DB        | $PATH_DATA/database | SQL database path                                         |
| PATH_LOG       | $PATH_DATA/logs     | Logs path                                                 |
| CONSOLE_LOG    | OFF                 | Should console log be enabled, ON to enable               |
| IP             | 127.0.0.1           | Bind address, use 0.0.0.0 for external access             |
| PORT           | 8080                | Bind port                                                 |
| DB_PEPPER      | None                | Database pepper used for password hashing if set          |

#### Example:
```shell
IP=0.0.0.0 CONSOLE_LOG=ON ./openltomanager
```

## Build
See: CONTRIBUTING.md