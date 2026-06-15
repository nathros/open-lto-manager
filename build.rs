#[cfg(not(debug_assertions))]
use std::process::Command;

fn main() {
    #[cfg(not(debug_assertions))]
    {
        // Release build only
        // TODO copy /scripts to: target/dx/openltomanager/release/web/

        // To generate SVG sprites run: external/generate-svg-sprites.sh

        // Combine CSS and JS files into single bundle
        match Command::new("bash")
            .arg("./external/generate-bundle.sh")
            .status()
        {
            Ok(exit_status) => {
                if !exit_status.success() {
                    println!("cargo:error=Bundle bad exit: {}", exit_status);
                    panic!("{}", exit_status); // Build failure
                }
            }
            Err(e) => {
                println!("cargo:error=Bundle error: {}", e);
                panic!("{}", e); // Build failure
            }
        }
    }
}
