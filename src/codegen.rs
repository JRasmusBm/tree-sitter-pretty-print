pub enum PrintNode {
    Literal {
        value: String,
    },
    Composite {
        format: String,
        children: Vec<PrintNode>,
    },
}

pub fn generate(node: &PrintNode) -> Result<String, &'static str> {
    match node {
        PrintNode::Literal { value } => return Ok(value.clone()),
        PrintNode::Composite { children, format } => {
            match generate(&children[0]) {
                Ok(child) => Ok(format.replace("{:}", &child)),
                Err(msg) => Err(msg)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_print_identifiers() {
        let node = PrintNode::Literal {
            value: String::from("hello"),
        };
        let want = String::from("hello");
        let got = generate(&node).unwrap_or(String::from("NOT IMPLEMENTED!"));

        assert_eq!(want, got);
    }

    #[test]
    fn can_print_wrapped_identifiers() {
        let node = PrintNode::Composite {
            format: String::from("'{:}'"),
            children: vec![PrintNode::Literal {
                value: String::from("hello"),
            }],
        };
        let want = String::from("'hello'");
        let got = generate(&node).unwrap_or(String::from("NOT IMPLEMENTED!"));

        assert_eq!(want, got);
    }
}
