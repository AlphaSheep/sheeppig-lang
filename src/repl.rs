use std::io::{stdin, stdout, Write};


use sheeppig::lexer::tokenize;
use sheeppig::parser::statement_parser::parse_statement;


pub fn repl() {
    println!("REPL v0.1.0");

    let input = stdin();

    loop {
        print!(":> ");
        stdout().flush().unwrap();


        let mut buffer = String::new();
        input.read_line(&mut buffer).unwrap();

        if buffer.trim() == "exit" {
            break;
        }

        let tokens = tokenize(&buffer);
        println!("\n -- Tokens: {:?} \n", tokens);

        let mut tokens = tokens.iter().peekable();
        let expression = parse_statement(&mut tokens);
        println!("\n -- Expression: {:?} \n", expression);
    }
}