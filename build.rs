#[cfg(not(debug_assertions))]
use std::process::Command;

#[cfg(not(debug_assertions))]
fn issue_cmd(command: &str) {
    match Command::new("bash").arg(command).status() {
        Ok(exit_status) => {
            if !exit_status.success() {
                println!("{} cargo:error=Bundle bad exit: {}", command, exit_status);
                panic!("{}", exit_status); // Build failure
            }
        }
        Err(e) => {
            println!("{} cargo:error=Bundle error: {}", command, e);
            panic!("{}", e); // Build failure
        }
    }
}

fn main() {
    #[cfg(not(debug_assertions))]
    {
        // Release build only
        // TODO copy /scripts to: target/dx/openltomanager/release/web/

        // To generate SVG sprites run: external/generate-svg-sprites.sh

        // Combine CSS and JS files into single bundle
        issue_cmd("./external/generate-bundle.sh");

        // Create assets/info-release.json which contains build info
        issue_cmd("./external/generate-about.sh");
    }
}
