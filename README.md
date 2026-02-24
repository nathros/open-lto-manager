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

### VsCode Setup
#### Install Toolchain
- Rust 1.92
- Dioxus build 0.7.3

https://dioxuslabs.com/learn/0.7/getting_started/

#### Install Recommend Extensions
paste @recommended in Extensions (Ctrl+Shift+X)
- rust-analyzer
- Code spell check
- Dioxus

### Final Checks
Run: `dev.env.check.sh` or VsCode task: Check::Env

This should pass with no errors