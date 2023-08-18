use crate::ast;
use itertools::Itertools;

pub fn generate(node: &ast::AstNode) -> Result<String, String> {
    match node {
        ast::AstNode::Literal { value } => return Ok(value.clone()),
        ast::AstNode::Composite {
            children,
            format,
            name: _,
        } => {
            match children
                .iter()
                .map(generate)
                .collect::<Result<Vec<String>, String>>()
            {
                Ok(child_strings) => {
                    return Ok(format
                        .split(ast::PLACEHOLDER)
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
    const DOES_NOT_MATTER: &str = "abc";

    #[test]
    fn can_print_identifiers() {
        let node = ast::AstNode::Literal {
            value: String::from("hello"),
        };
        let want = String::from("hello");
        let got = generate(&node).unwrap_or(String::from("NOT IMPLEMENTED!"));

        assert_eq!(want, got);
    }

    #[test]
    fn can_print_wrapped_identifiers() {
        let node = ast::AstNode::Composite {
            name: String::from(DOES_NOT_MATTER),
            format: format!("'{}'", ast::PLACEHOLDER),
            children: vec![ast::AstNode::Literal {
                value: String::from("hello"),
            }],
        };
        let want = String::from("'hello'");
        let got = generate(&node).unwrap_or(String::from("NOT IMPLEMENTED!"));

        assert_eq!(want, got);
    }

    #[test]
    fn a_more_complicated_example() {
        let node = ast::AstNode::Composite {
            name: String::from(DOES_NOT_MATTER),
            format: format!(
                "if ({}) {{
    {}
}} else {{
    {}
}}
",
                ast::PLACEHOLDER,
                ast::PLACEHOLDER,
                ast::PLACEHOLDER,
            ),
            children: vec![
                ast::AstNode::Composite {
                    name: String::from(DOES_NOT_MATTER),
                    format: format!("{} < {}", ast::PLACEHOLDER, ast::PLACEHOLDER),
                    children: vec![
                        ast::AstNode::Literal {
                            value: String::from("a"),
                        },
                        ast::AstNode::Literal {
                            value: String::from("2"),
                        },
                    ],
                },
                ast::AstNode::Composite {
                    name: String::from(DOES_NOT_MATTER),
                    format: format!("return {};", ast::PLACEHOLDER),
                    children: vec![ast::AstNode::Literal {
                        value: String::from("true"),
                    }],
                },
                ast::AstNode::Composite {
                    name: String::from(DOES_NOT_MATTER),
                    format: format!("return {};", ast::PLACEHOLDER),
                    children: vec![ast::AstNode::Literal {
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
