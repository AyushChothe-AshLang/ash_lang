use std::env::args;
use std::fs;

use ash_lang::formatter::Formatter;
use ash_lang::interpreter::Interpreter;
use ash_lang::lexer::Lexer;
use ash_lang::parser::Parser;

fn main() -> Result<(), String> {
    let args = args().collect::<Vec<String>>();

    let cmd;
    let file;
    if args.len() == 2 {
        cmd = "run".to_string();
        file = args
            .get(1)
            .expect("Expected command or file as second argument")
            .to_string();
    } else if args.len() == 3 {
        cmd = args
            .get(1)
            .expect("Expected command as second argument")
            .to_string();
        file = args
            .get(2)
            .expect("Expected file as third argument")
            .to_string();
    } else {
        return Err("Invalid Arguments".to_string());
    }

    let code = fs::read_to_string(file).map_err(|x| format!("{}", x))?;

    if cmd == *"run" {
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;

        // Executes the Code
        let mut interpreter = Interpreter::new(ast);
        interpreter.eval();

        return Ok(());
    } else if cmd == *"analyze" {
        // Analyzes the Code
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let _ = parser.parse()?;

        return Ok(());
    } else if cmd == *"fmt" {
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;
        let mut formatter = Formatter::new(4);

        println!("{}", formatter.format(ast));
        return Ok(());

        // return Err("Formatter is in development 😎".to_string());
    }

    Err("Something went wrong".to_string())
}
