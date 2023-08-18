use crate::ast;
use itertools::Itertools;

pub fn generate(node: &ast::AstNode) -> Result<String, String> {
    match node {
        ast::AstNode::Literal { value } => return Ok(format!("(Literal \"{}\")", value.clone())),
        ast::AstNode::Composite {
            name,
            children,
            format,
        } => {
            let maybe_children = match children
                .iter()
                .map(generate)
                .collect::<Result<Vec<String>, String>>()
            {
                Ok(child_strings) => {
                    return Ok(child_strings.join("\n"));
                }
                Err(msg) => Err(msg),
            };

            if let Ok(children) = maybe_children {
                return Ok(format!("(Composite {})", children));
            } else {
                return maybe_children
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_print_identifiers() {
        let node = ast::AstNode::Literal {
            value: String::from("hello"),
        };
        let want = String::from("(Literal \"hello\")");
        let got = generate(&node).unwrap_or(String::from("NOT IMPLEMENTED!"));

        assert_eq!(want, got);
    }

    #[test]
    fn can_print_wrapped_identifiers() {
        let node = ast::AstNode::Composite {
            name: String::from("String"),
            format: format!("'{}'", ast::PLACEHOLDER),
            children: vec![ast::AstNode::Literal {
                value: String::from("hello"),
            }],
        };
        let want = format!(
            "(Composite
            name=\"String\"
            format=\"'{}'\"
            (
                (Literal \"hello\")
            )
        )",
            ast::PLACEHOLDER
        );
        let got = generate(&node).unwrap_or(String::from("NOT IMPLEMENTED!"));

        assert_eq!(want, got);
    }
}
