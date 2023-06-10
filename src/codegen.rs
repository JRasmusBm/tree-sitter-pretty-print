pub enum AstKind {
    Literal,
}

pub struct AstNode {
    pub kind: AstKind,
    pub format: String,
}

pub fn generate(node: &AstNode) -> Option<String> {
    match node.kind {
        AstKind::Literal => return Some(node.format.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_print_identifiers() {
        let node = AstNode {
            kind: AstKind::Literal,
            format: String::from("hello"),
        };
        let want = String::from("hello");
        let got = generate(&node).unwrap_or(String::from("NOT IMPLEMENTED!"));

        assert_eq!(want, got);
    }
}
