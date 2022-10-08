pub mod built_in;
pub mod interpreter;
pub mod lexer;
pub mod nodes;
pub mod parser;
pub mod scope;
pub mod tokens;
pub mod utils;
pub mod values;

use std::fs;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;
fn main() {
    // let code = String::from();
    let code = fs::read_to_string("D:/RustProjects/math_eval/src/code.ash")
        .expect("Should have been able to read the file");
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize();
    println!("{:?}", tokens);

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    // print!("{:?}", ast);

    let mut interpreter = Interpreter::new(ast);

    interpreter.eval();
}
