#[cfg(test)]
mod tests {
    use crate::util;

    #[test]
    fn test_get_rust_files() {
        let files = util::rust_files(".").expect("rust files not found.");
        assert_eq!(3, files.len());
    }
}
