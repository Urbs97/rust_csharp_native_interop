use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

fn main() {
    // Determine if we are in release mode or debug mode
    let is_release = env::var("PROFILE").unwrap() == "release";

    // Detect the target operating system
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    // Determine the file extension and subdirectory based on the OS
    let (file_extension, subdirectory) = if target_os == "windows" {
        ("dll", "win-x64")
    } else if target_os == "linux" {
        ("so", "linux-x64")
    } else {
        panic!("Unsupported target OS: {}", target_os);
    };

    // Get the current directory to construct the absolute source path
    let current_dir = env::current_dir().unwrap();
    let source_file = current_dir.parent().unwrap().join(format!(
        "NativeCSharp/bin/Release/{}/publish/NativeCSharp.{}",
        subdirectory, file_extension
    ));

    // Get the appropriate target directory
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = PathBuf::from(out_dir)
        .ancestors()
        .nth(3)
        .expect("Failed to get target directory")
        .to_path_buf();

    let dest_file = if is_release {
        target_dir.join(format!("NativeCSharp.{}", file_extension))
    } else {
        target_dir.join(format!("NativeCSharp.{}", file_extension))
    };

    // Check if the destination file exists and compare the modification times
    let should_copy = match fs::metadata(&dest_file) {
        Ok(dest_metadata) => {
            let dest_modified = dest_metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
            let src_modified = fs::metadata(&source_file)
                .and_then(|meta| meta.modified())
                .unwrap_or(SystemTime::UNIX_EPOCH);
            src_modified > dest_modified
        }
        Err(_) => true, // File doesn't exist, so we should copy it
    };

    // Copy the file only if it has changed
    if should_copy {
        fs::create_dir_all(dest_file.parent().unwrap()).unwrap();
        fs::copy(&source_file, &dest_file)
            .expect(&format!("Failed to copy the file from {} to {}", source_file.display(), dest_file.display()));
        println!("Copied {} to {}", source_file.display(), dest_file.display());
    } else {
        println!("No need to copy; the destination file is up to date.");
    }

    // Print a message to Cargo output
    println!("cargo:rerun-if-changed={}", source_file.display());
}
