use itertools::Itertools;

const PLACEHOLDER: &str = "{::}";

pub enum PrintNode {
    Literal {
        value: String,
    },
    Composite {
        format: String,
        children: Vec<PrintNode>,
    },
}

pub fn generate(node: &PrintNode) -> Result<String, String> {
    match node {
        PrintNode::Literal { value } => return Ok(value.clone()),
        PrintNode::Composite { children, format } => {
            match children
                .iter()
                .map(generate)
                .collect::<Result<Vec<String>, String>>()
            {
                Ok(child_strings) => {
                    return Ok(format
                        .split(PLACEHOLDER)
                        .interleave(child_strings.iter().map(|s| s.as_str()))
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>()
                        .join(""));
                }
                Err(msg) => Err(msg),
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
            format: format!("'{}'", PLACEHOLDER),
            children: vec![PrintNode::Literal {
                value: String::from("hello"),
            }],
        };
        let want = String::from("'hello'");
        let got = generate(&node).unwrap_or(String::from("NOT IMPLEMENTED!"));

        assert_eq!(want, got);
    }

    #[test]
    fn a_more_complicated_example() {
        let node = PrintNode::Composite {
            format: format!(
                "if ({}) {{
    {}
}} else {{
    {}
}}
",
                PLACEHOLDER, PLACEHOLDER, PLACEHOLDER,
            ),
            children: vec![
                PrintNode::Composite {
                    format: format!("{} < {}", PLACEHOLDER, PLACEHOLDER),
                    children: vec![
                        PrintNode::Literal {
                            value: String::from("a"),
                        },
                        PrintNode::Literal {
                            value: String::from("2"),
                        },
                    ],
                },
                PrintNode::Composite {
                    format: format!("return {};", PLACEHOLDER),
                    children: vec![PrintNode::Literal {
                        value: String::from("true"),
                    }],
                },
                PrintNode::Composite {
                    format: format!("return {};", PLACEHOLDER),
                    children: vec![PrintNode::Literal {
                        value: String::from("false"),
                    }],
                },
            ],
        };
        let want = String::from(
            "if (a < 2) {
    return true;
} else {
    return false;
}
",
        );
        let got = generate(&node).unwrap_or(String::from("NOT IMPLEMENTED!"));

        assert_eq!(want, got);
    }
}
