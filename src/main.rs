mod codegen;

fn main() {
    if let Ok(result) = codegen::generate(&codegen::PrintNode::Literal {
        value: String::from("hello"),
    }) {
        println!("{}", result);
    } else {
        println!("Formatting failed!")
    }
}
