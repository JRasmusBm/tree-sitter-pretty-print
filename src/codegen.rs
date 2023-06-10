pub enum AstKind {
    Literal,
    Wrapped,
}

pub struct AstNode {
    pub kind: AstKind,
    pub format: String,
    pub children: Vec<AstNode>,
}

pub fn generate(node: &AstNode) -> Option<String> {
    match node.kind {
        AstKind::Literal => return Some(node.format.clone()),
        AstKind::Wrapped => {
            if let Some(child) = generate(&node.children[0]) {
                return Some(node.format.replace("{:1}", &child));
            }
            return None;
        }
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
            children: vec![],
        };
        let want = String::from("hello");
        let got = generate(&node).unwrap_or(String::from("NOT IMPLEMENTED!"));

        assert_eq!(want, got);
    }

    #[test]
    fn can_print_wrapped_identifiers() {
        let node = AstNode {
            kind: AstKind::Wrapped,
            format: String::from("'{:1}'"),
            children: vec![AstNode {
                kind: AstKind::Literal,
                format: String::from("hello"),
                children: vec![],
            }],
        };
        let want = String::from("'hello'");
        let got = generate(&node).unwrap_or(String::from("NOT IMPLEMENTED!"));

        assert_eq!(want, got);
    }
}
