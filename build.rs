use std::{
    fs::{self, File, OpenOptions},
    io::{self, BufRead, BufReader, Error},
};

fn main() {
    #[cfg(not(debug_assertions))]
    {
        // Release build only

        // TODO combine JS into single file
        // TODO copy /scripts to: target/dx/openltomanager/release/web/

        // To generate SVG sprites run: external/generate-svg-sprites.sh

        // Combine CSS files into single bundle
        if let Err(e) = bundle_css() {
            println!("cargo:error=Bundle CSS: {}", e);
            panic!("{}", e); // Build failure
        }
    }
}

#[allow(dead_code)]
fn bundle_css() -> Result<(), Error> {
    let file = File::open("src/frontend/assets.rs")?;
    let reader = BufReader::new(file);

    let bundle_path = "assets/bundle.css";
    let mut bundle_file = OpenOptions::new()
        .write(true) // Open to write
        .create(true) // Create if it doesn't exist
        .truncate(true) // Overwrite existing content
        .open(bundle_path)?;
    println!("cargo:info=Create CSS bundle: {}", bundle_path);

    let mut found_css_array = false;

    for line in reader.lines() {
        let line = line?;

        if found_css_array {
            if line.contains("];") {
                return Ok(());
            } else if let Some(start_index) = line.find('"')
                && let Some(end_index) = line.rfind('"')
            {
                let asset_path = &line[start_index + 2..end_index];
                println!("cargo:info=Append CSS bundle: {}", asset_path);
                let mut asset_file = fs::OpenOptions::new().read(true).open(asset_path)?;
                io::copy(&mut asset_file, &mut bundle_file)?;
            }
        } else if line.contains("#[cfg(debug_assertions)] // Debug build") {
            found_css_array = true;
        }
    }
    Err(Error::other("Failed to find CSS assets array"))
}
