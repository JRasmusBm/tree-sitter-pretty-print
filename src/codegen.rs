mod codegen {
        
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn can_print_identifiers() {
            let want = "hello" ;
            let got = codegen();

            assert_eq!(want, got);
        }
    }

    fn codegen() -> String {
        return "hello".to_string();
    }
}
