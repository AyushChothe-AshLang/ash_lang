pub mod built_in;
pub mod interpreter;
pub mod lexer;
pub mod nodes;
pub mod parser;
pub mod scope;
pub mod tokens;
pub mod utils;
pub mod values;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn run(code: String) -> Result<String, String> {
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize()?;
    // println!("{:?}", tokens);

    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    // print!("{:?}", ast);

    let mut interpreter = Interpreter::new(ast);

    Ok(format!("{}", interpreter.eval()))
}
