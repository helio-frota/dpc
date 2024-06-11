#[cfg(test)]
mod tests {
    use crate::util;

    #[test]
    fn test_get_rust_files() {
        let files = util::rust_files(".").expect("rust files not found.");
        assert_eq!(3, files.len());
    }

    #[test]
    fn test_get_content() {
        let files = util::rust_files(".").expect("rust files not found.");
        let path_as_str = files[0].to_string_lossy();
        let content = util::content(&path_as_str);
        assert!(content.unwrap().contains("use"));
    }

    #[test]
    fn test_get_code_blocks() {
        let files = util::rust_files(".").expect("rust files not found.");
        let path_as_str = files[0].to_string_lossy();
        let content = util::content(&path_as_str);
        let blocks = util::code_blocks(content.unwrap().as_str());
        assert!(!blocks[0].is_empty());

        let mut use_found = false;
        for b in blocks {
            if b.contains("use") {
                use_found = true;
            }
        }
        assert!(!use_found);
    }

    #[test]
    fn test_block_counter() {
        let files = util::rust_files(".").expect("rust files not found.");
        let path_as_str = files[0].to_string_lossy();
        let content = util::content(&path_as_str);
        let blocks = util::code_blocks(content.unwrap().as_str());
        // kind of dumb unit test since I'm already discarding empty lines...
        for cb in &blocks {
            let f = util::block_counter(cb);
            assert!(f.0 > 0);
            assert!(f.1 > 0);
        }
    }

    #[test]
    fn test_get_similar() {
        let files = util::rust_files(".").expect("rust files not found.");
        let path_as_str = files[0].to_string_lossy();
        let content = util::content(&path_as_str);
        let blocks = util::code_blocks(content.unwrap().as_str());
        // Intentionally lowering the threshold to test something
        let similar_found = util::similar(blocks, 0.10);
        assert!(!similar_found.is_empty());
        for s in &similar_found {
            assert!(!s.0.is_empty());
            assert!(!s.1.is_empty());
            assert!(s.2 > 0.10);
        }
    }
}
