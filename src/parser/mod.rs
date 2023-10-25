use crate::tokens::Token;

mod utils;

mod module_parser;
mod import_parser;
mod function_parser;
pub mod statement_parser;
mod expression_parser;
mod atomic_parser;


pub fn parse(tokens: &[Token]) -> crate::tree::Module {
    let mut input = tokens.iter().peekable();

    module_parser::parse_module(&mut input)
}
