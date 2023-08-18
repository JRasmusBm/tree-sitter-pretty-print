pub const PLACEHOLDER: &str = "{::}";

pub enum AstNode {
    Literal {
        value: String,
    },
    Composite {
        format: String,
        children: Vec<AstNode>,
    },
}
