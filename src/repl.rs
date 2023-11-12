use std::io::{stdin, stdout, Write};


use sheeppig::lexer::tokenize;
use sheeppig::parser::parse;


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

        let expression = parse(&tokens);
        println!("\n -- Expression: {:?} \n", expression);
    }
}