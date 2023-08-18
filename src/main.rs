mod ast;
mod codegen;

fn main() {
    if let Ok(result) = codegen::source::generate(&ast::AstNode::Literal {
        value: String::from("hello"),
    }) {
        println!("{}", result);
    } else {
        println!("Formatting failed!")
    }
}
