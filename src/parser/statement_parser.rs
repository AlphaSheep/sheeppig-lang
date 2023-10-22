use core::panic;
use std::iter::Peekable;
use std::slice::Iter;

use crate::elements::{ Identifier, Literal, Operator, Keyword };
use crate::tokens::Token;
use crate::tree::{
    StatementBlock, Statement,
    Expression, AtomicExpression, ParenthesizedExpression, FunctionCallExpression
};

use crate::parser::utils::{ handle_parse_error, handle_parse_error_for_option };
use crate::parser::expression_parser::parse_expression;


pub fn parse_statement_block(tokens: &mut Peekable<Iter<Token>>) -> StatementBlock {
    if tokens.next() != Some(&Token::OpenBrace) {
        handle_parse_error_for_option::<()>("Expected open brace after function signature, found {:?}", tokens.peek());
    }

    let mut statements = vec![];

    while let Some(token) = tokens.peek() {
        match token {
            Token::Newline => { tokens.next(); },
            Token::CloseBrace => { tokens.next(); break },
            _ => statements.push(parse_statement(tokens)),
        }
    }

    StatementBlock {
        statements,
    }
}


fn parse_statement(tokens: &mut Peekable<Iter<Token>>) -> Statement {
    let statement_tokens = consume_statement_tokens(tokens);



    panic!("Not implemented");
}


fn consume_statement_tokens(tokens: &mut Peekable<Iter<Token>>) -> Vec<Token> {
    let mut statement_tokens = vec![];

    while let Some(token) = tokens.next() {
        match token {
            Token::Newline => break,
            _ => statement_tokens.push(token.clone()),
        }
    }

    statement_tokens
}

