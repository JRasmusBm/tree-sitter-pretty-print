pub enum AstNode {
    Literal {
        value: String,
    },
    Wrapped {
        format: String,
        children: Vec<AstNode>,
    },
}

pub fn generate(node: &AstNode) -> Option<String> {
    match node {
        AstNode::Literal { value } => return Some(value.clone()),
        AstNode::Wrapped { children, format } => {
            if let Some(child) = generate(&children[0]) {
                return Some(format.replace("{:1}", &child));
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
        let node = AstNode::Literal {
            value: String::from("hello"),
        };
        let want = String::from("hello");
        let got = generate(&node).unwrap_or(String::from("NOT IMPLEMENTED!"));

        assert_eq!(want, got);
    }

    #[test]
    fn can_print_wrapped_identifiers() {
        let node = AstNode::Wrapped {
            format: String::from("'{:1}'"),
            children: vec![AstNode::Literal {
                value: String::from("hello"),
            }],
        };
        let want = String::from("'hello'");
        let got = generate(&node).unwrap_or(String::from("NOT IMPLEMENTED!"));

        assert_eq!(want, got);
    }
}
