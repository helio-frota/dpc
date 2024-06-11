use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::{fs, io};

use strsim::sorensen_dice;

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

pub fn code_blocks(content: &str) -> Vec<String> {
    // Changes the content to a vector lines.
    let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    let mut i = 0;
    let mut code_blocks: Vec<String> = Vec::new();
    // loop the lines and discard some stuff.
    while i < lines.len() {
        // discards empty line
        if lines[i].trim().is_empty() {
            i += 1;
            continue;
        }

        // discards imports / use stuff
        if lines[i].trim().contains("use crate") {
            i += 1;
            continue;
        }

        // discards imports / use stuff
        if lines[i].trim().contains("use") {
            i += 1;
            continue;
        }

        // discards imports / use stuff
        if lines[i].trim().contains("pub mod") {
            i += 1;
            continue;
        }

        // discards comments
        if lines[i].trim().contains("//") {
            i += 1;
            continue;
        }

        // discards comments
        if lines[i].trim().contains("///") {
            i += 1;
            continue;
        }

        // Creates a temporary code-block vector of lines.
        let mut block_lines: Vec<String> = Vec::new();
        block_lines.push(lines[i].clone());

        let mut j = i + 1;
        while j < lines.len() && !lines[j].trim().is_empty() {
            block_lines.push(lines[j].clone());
            j += 1;
        }

        if block_lines.len() > 1 {
            code_blocks.push(block_lines.join("\n"));
        }

        i = j;
    }
    code_blocks
}

// This counts lines and chars from code-block
// to make opinionated checks later.
pub fn block_counter(cb: &str) -> (i16, i16) {
    let mut chars = 0;
    let mut lines = 0;

    for line in cb.lines() {
        let trimmed = line.trim();
        chars += trimmed.len() as i16;
        lines += 1;
    }

    (chars, lines)
}

pub fn find_similar_blocks(blocks: Vec<String>) -> Vec<(String, String, f64)> {
    let mut result = Vec::new();
    let threshold = 0.98;

    for i in 0..blocks.len() {
        for j in i + 1..blocks.len() {
            let b1 = blocks[i].as_str();
            let b2 = blocks[j].as_str();
            let similarity = sorensen_dice(b1, b2);

            if similarity > threshold {
                let t = (b1.to_string(), b2.to_string(), similarity);
                result.push(t);
            }
        }
    }

    result
}

pub fn report(similar_blocks: Vec<(String, String, f64)>) {
    if !similar_blocks.is_empty() {
        println!("# Duplicrabs\n");

        for (idx, s) in similar_blocks.iter().enumerate() {
            let mut b1: Vec<&str> = s.0.split('\n').collect();
            let f1 = b1.pop();
            let mut b2: Vec<&str> = s.1.split('\n').collect();
            let f2 = b2.pop();

            println!("### crab {}\n", idx + 1);

            if s.2 == 1.0 {
                println!("> [!TIP]\n> Exactly the same\n");
            } else {
                println!("> [!WARNING]\n> Almost the same\n");
            }

            println!("```rust");
            println!("{}", b1.join("\n"));
            println!("```\n");
            println!("`{}`\n", f1.unwrap_or("n/a"));
            println!("```rust");
            println!("{}", b2.join("\n"));
            println!("```\n");
            println!("`{}`\n", f2.unwrap_or("n/a"));
        }
    }
}
