use ass_parser::AssFile;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utf8_bom() {
        assert_eq!(AssFile::from_file("./examples/gbc_e1.ass").is_ok(), true);
    }

    #[test]
    fn test_fail_utf8_bom() {
        assert_eq!(AssFile::from_file("./examples/empty.ass").is_ok(), false);
    }

}
