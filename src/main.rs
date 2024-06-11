use std::io;

mod util;

#[cfg(test)]
mod test;

fn main() -> Result<(), io::Error> {
    let rust_files = util::rust_files("/home/heliofrota/Desktop/tc/trustify/")?;
    let mut filtered_code_blocks: Vec<String> = Vec::new();
    for r in &rust_files {
        let r_as_string = r.to_string_lossy();
        let the_index = r_as_string
            .find("trustify")
            .unwrap_or_else(|| r_as_string.len());
        let file_name = &r_as_string[the_index..];

        let content = util::content(r.to_string_lossy().into_owned().as_str())?;
        let cbs = util::code_blocks(content.as_str());

        for cb in &cbs {
            let f = util::block_counter(cb);
            if f.0 >= 150 && f.1 >= 2 {
                let b = format!("{}{}{}", cb, "\n", file_name);
                filtered_code_blocks.push(b);
            }
        }
    }

    let similar_blocks = util::find_similar_blocks(filtered_code_blocks);
    util::report(similar_blocks);

    Ok(())
}
