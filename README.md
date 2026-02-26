## Dependencies
### Hardware

### Software
For LTFS support you will need LTO-5 or newer and have a LTFS driver installed, [OpenLTFS](https://github.com/LinearTapeFileSystem/ltfs) is recommended, others are [HPE LTFS](https://github.com/nix-community/hpe-ltfs) or [IBM LTFS](https://www.ibm.com/docs/en/spectrum-archive-le?topic=tools-downloading-ltfs).
Tar support will need `mt` installed. For most distributions access to tape devices `/dev/nst[x]` and `/dev/st[x]` the user will need to be part of the `tape` group (Arch is `storage`).

## Install
### Recommended dependencies
```shell
bash <(curl -L https://raw.githubusercontent.com/nathros/open-lto-manager/main/scripts/deps-install.sh)
```


### Environment variables
| Variable       | Default             | Description                                               |
| -------------- | ------------------- | --------------------------------------------------------- |
| PATH_DATA      | data                | Application data path which include logs and the database |
| PATH_DB        | $PATH_DATA/database | SQL database path                                         |
| PATH_LOG       | $PATH_DATA/logs     | Logs path                                                 |
| CONSOLE_LOG    | OFF                 | Should console log be enabled, ON to enable               |
| IP             | 127.0.0.1           | Bind address, use 0.0.0.0 for external access             |
| PORT           | 8080                | Bind port                                                 |

Example: IP="0.0.0.0" CONSOLE_LOG="ON" ./openltomanager

## Build
### VsCode Setup
#### Install Toolchains
- Rust 1.92
- Dioxus build 0.7.3

https://dioxuslabs.com/learn/0.7/getting_started/

#### Install Recommend Extensions
paste @recommended in Extensions (Ctrl+Shift+X)
- rust-analyzer
- Code spell check
- Dioxus

### Final Checks
Run: `dev.env.check.sh` or VsCode task: `Check::Env`

This should pass with no errors

### Local/debug Build
Run: `dx serve` or VsCode task: `Serve::localhost`

### Release Build
Run: `dx build --web --release` or VsCode task: `Build::Release`