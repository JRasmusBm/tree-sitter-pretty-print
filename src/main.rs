mod codegen;

fn main() {
    if let Some(result) = codegen::generate(&codegen::AstNode {
        kind: codegen::AstKind::Literal,
        format: String::from("hello"),
    }) {
        println!("{}", result);
    } else {
        println!("Formatting failed!")
    }
}
