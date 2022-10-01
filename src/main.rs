pub mod interpreter;
pub mod lexer;
pub mod nodes;
pub mod parser;
pub mod tokens;
pub mod values;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

fn main() {
    let code = String::from("(((2^2)*(3+2)*2)/2)^2");

    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize();
    // println!("{:?}", tokens);

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    // print!("{:?}", ast);

    let interpreter = Interpreter::new(ast);

    println!("{:?}", interpreter.eval().get_literal());
}
