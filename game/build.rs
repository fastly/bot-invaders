use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // This runs after the build
   println!("cargo:rerun-if-changed=src/");

    // Get the target directory
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir).ancestors().nth(3).unwrap();
    
    // Source WASM file
    let wasm_src = target_dir.join("space_invaders.wasm");
    
    // Destination (adjust path as needed)
    let wasm_dest = Path::new("../space_invaders.wasm");
    
    if wasm_src.exists() {
        if let Err(e) = fs::copy(&wasm_src, &wasm_dest) {
            println!("cargo:warning=Failed to copy WASM file: {}", e);
        } else {
            println!("cargo:warning=WASM file copied to {}", wasm_dest.display());
        }
    }
}