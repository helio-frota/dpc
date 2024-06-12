use std::{env, io, path::Path, process};

mod util;

#[cfg(test)]
mod test;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: dpc .  or dpc /home/foobar/rust_project");
        process::exit(1);
    }

    let dir = &args[1];
    let last_dir = Path::new(dir)
        .file_name()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or("Something happened and who cares...");

    let rust_files = util::rust_files(dir)?;

    let mut filtered_code_blocks: Vec<String> = Vec::new();

    for r in &rust_files {

        // Some stuff to show the file name on duplicrabs report later.
        let r_as_string = r.to_string_lossy();
        let the_index = r_as_string
            .find(last_dir)
            .unwrap_or_else(|| r_as_string.len());
        let file_name = &r_as_string[the_index..];

        let content = util::content(r.to_string_lossy().into_owned().as_str())?;
        let cbs = util::code_blocks(content.as_str());

        for cb in &cbs {
            let f = util::block_counter(cb);
            // at least 150+ chars and 2+ lines...
            if f.0 >= 150 && f.1 >= 2 {
                let b = format!("{}{}{}", cb, "\n", file_name);
                filtered_code_blocks.push(b);
            }
        }
    }

    // Like the 150 and 2 above ^, this 0.98 is total opinionated...
    // if we lower this we have more "almost the same" stuff.
    let similar_blocks = util::similar(filtered_code_blocks, 0.98);
    util::report(similar_blocks);

    Ok(())
}
