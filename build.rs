fn main() {
    #[cfg(not(debug_assertions))]
    {
        // Release build
        // TODO combine SVGs into layered sprite
        // TODO combine CSS into single file
        // TODO combine JS into single file
        // copy /scripts to: target/dx/openltomanager/release/web/
    }
}
