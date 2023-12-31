use std::iter::Peekable;
use std::slice::Iter;

use crate::elements::{Identifier, Keyword};
use crate::tokens::Token;
use crate::tree::{self, Module};

use crate::parser::utils::handle_parse_error;
use crate::parser::import_parser::parse_using_block;
use crate::parser::function_parser::parse_function_block;

use super::statement_parser::parse_statements_until_end_of_module;


pub fn parse_module(tokens: &mut Peekable<Iter<Token>>) -> Module {

    let mut has_import = false;
    let mut has_function = false;
    let mut has_statements = false;

    let mut imports: Vec<tree::Import> = vec![];
    let mut functions: Vec<tree::Function> = vec![];
    let mut statements: tree::StatementBlock = tree::StatementBlock::empty();

    while let Some(token) = tokens.peek() {
        match token {
            Token::Newline => {
                tokens.next();
            },

            Token::Keyword(Keyword::Using) => {
                if !has_import && !has_function && !has_statements {
                    tokens.next();
                    imports.push(parse_using_block(tokens));
                    has_import = true;
                } else {
                    handle_parse_error::<()>("Only one using block is allowed and must be at the top of the module", token);
                }
            },

            Token::Keyword(Keyword::Function) => {
                if !has_statements {
                    tokens.next();
                    functions.push(parse_function_block(tokens));
                    has_function = true;
                } else {
                    handle_parse_error::<()>("Function blocks must come before any statements", token);
                }
            }

            Token::EndOfModule => break,

            _ => {
                statements = parse_statements_until_end_of_module(tokens);
                if statements.statements.len() > 0 {
                    has_statements = true;
                }
            },
        }
    }

    Module {
        name: Identifier::Simple("main".to_string()),
        imports: imports,
        functions: functions,
        statements: statements,
    }
}
