pub const PLACEHOLDER: &str = "{::}";

pub enum AstNode {
    Literal {
        value: String,
    },
    Composite {
        name: String,
        format: String,
        children: Vec<AstNode>,
    },
}
