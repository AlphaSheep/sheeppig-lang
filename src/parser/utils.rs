use std::fmt::Debug;

use crate::tokens::Token;


pub fn handle_parse_error<T>(message: &str, token: &Token) -> T {
    panic!("Parse error: {}\n\n Found {:?}\n", message, token);
}

pub fn handle_parse_error_for_option<T>(message: &str, token: Option<&impl Debug>) -> T {
    match token {
        Some(t) => panic!("Parse error: {}\n\n Found {:?}\n", message, t),
        None => panic!("Parse error: {}\n\n Found EOF\n", message)
    };
}