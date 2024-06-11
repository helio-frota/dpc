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
}
