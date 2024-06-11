use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::{fs, io};

/// This function gets all the .rs files from a directory, and returns a
/// vector with all the paths of the .rs files.
pub fn rust_files(dir: &str) -> Result<Vec<PathBuf>, io::Error> {
    let mut files = Vec::new();
    // Gets all the entries from 'current|argument' dir.
    let entries = fs::read_dir(dir)?;

    for e in entries {
        let entry = e?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(file_name) = path.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    if file_name_str.contains("target") || file_name_str.contains("test") {
                        continue;
                    }
                }
            }

            // Take a look at sub dir
            let temp = rust_files(&path.to_string_lossy())?;
            // .rs found ? Then add to the vector.
            files.extend(temp);
        } else if let Some(file_name) = path.file_name() {
            // Not a dir and .rs
            if file_name.to_string_lossy().ends_with(".rs") {
                // Then adds to the vector.
                files.push(path);
            }
        }
    }

    Ok(files)
}

// Reads file content.
pub fn content(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut c = String::new();
    f.read_to_string(&mut c)?;
    Ok(c)
}
