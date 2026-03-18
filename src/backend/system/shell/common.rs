use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::{ChildStderr, ChildStdout},
    sync::broadcast::Sender,
};
use tracing::error;

// Stream both stdout and stderr to subscribers of Sender
pub async fn shell_output_default(
    sender: Sender<String>,
    stdout: ChildStdout,
    stderr: ChildStderr,
) {
    let sender_std_out = sender.clone();
    let stdout_spawn = tokio::spawn(async move {
        let stdout_reader = BufReader::new(stdout);
        let mut stdout_lines = stdout_reader.lines();
        loop {
            match stdout_lines.next_line().await {
                Ok(line_opt) => {
                    if let Some(line) = line_opt {
                        let _ = sender_std_out.send(line);
                    } else {
                        break; // No more data
                    }
                }
                Err(e) => {
                    error!("Failed to get stdout {}", e);
                    break;
                }
            }
        }
    });

    let stderr_spawn = tokio::spawn(async move {
        let stderr_reader = BufReader::new(stderr);
        let mut stderr_lines = stderr_reader.lines();
        loop {
            match stderr_lines.next_line().await {
                Ok(line_opt) => {
                    if let Some(line) = line_opt {
                        let _ = sender.send(line);
                    } else {
                        break; // No more data
                    }
                }
                Err(e) => {
                    error!("Failed to get stderr {}", e);
                    break;
                }
            }
        }
    });

    if let Err(e) = stdout_spawn.await {
        error!("Failed to wait for check stdout: {}", e);
    }
    if let Err(e) = stderr_spawn.await {
        error!("Failed to wait for check stderr: {}", e);
    }
}
