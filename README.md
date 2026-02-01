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