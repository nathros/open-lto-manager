use std::{
    io::{BufRead, BufReader},
    process::Stdio,
};

use crate::shared::error::ErrorStr;

// Only for short lived commands
pub fn shell_command_output_blocking(
    command: &str,
    args: Vec<&str>,
) -> Result<(Vec<String>, Vec<String>), ErrorStr> {
    match std::process::Command::new(command)
        .args(args)
        .stdout(Stdio::piped()) // Hide output from console
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(mut child) => {
            // Need to wait otherwise child will be marked as defunct
            if let Some(e) = child.wait().err() {
                Err(format!("Failed to wait for: {} with error: {}", command, e))
            } else {
                if let Some(stderr) = child.stderr.take()
                    && let Some(stdout) = child.stdout.take()
                {
                    let stdout_reader = BufReader::new(stdout);
                    let mut stdout_lines = stdout_reader.lines();

                    let stderr_reader = BufReader::new(stderr);
                    let mut stderr_lines = stderr_reader.lines();

                    let mut stdout_result = vec![];
                    let mut stderr_result = vec![];

                    for line_result in stdout_lines.by_ref() {
                        match line_result {
                            Ok(line) => stdout_result.push(line.clone()),
                            Err(_) => break,
                        }
                    }

                    for line_result in stderr_lines.by_ref() {
                        match line_result {
                            Ok(line) => stderr_result.push(line.clone()),
                            Err(_) => break,
                        }
                    }

                    return Ok((stdout_result, stderr_result));
                }
                Err(format!(
                    "Shell command failed to get stdout/err: {}",
                    command
                ))
            }
        }
        Err(e) => {
            if std::io::ErrorKind::NotFound != e.kind() {
                Err(format!("Spawn error: {}", e))
            } else {
                Err(format!("Command not found: {}", command))
            }
        }
    }
}
