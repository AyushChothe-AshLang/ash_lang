use std::fs;

use ash_lang::interpreter::Interpreter;
use ash_lang::lexer::Lexer;
use ash_lang::parser::Parser;
fn main() -> Result<(), String> {
    // let code = String::from("");
    // let code = fs::read_to_string("/mnt/d/RustProjects/math_eval/src/code.ash")
    let code =
        fs::read_to_string("./src/code.ash").expect("Should have been able to read the file");
    // println!("{}", code);
    let mut lexer = Lexer::new(code);

    let tokens = lexer.tokenize()?;
    // println!("{:?}", tokens);

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    // print!("{:?}", ast);

    let mut interpreter = Interpreter::new(ast);

    interpreter.eval();
    Ok(())
}
