pub fn generate() -> String {
    return "hello".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_print_identifiers() {
        let want = "hello";
        let got = generate();

        assert_eq!(want, got);
    }
}
