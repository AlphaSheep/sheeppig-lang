use core::panic;
use std::iter::Peekable;
use std::slice::Iter;

use crate::elements::{ Identifier, Literal, Operator, Keyword };
use crate::tokens::Token;
use crate::tree::{
    Expression, AtomicExpression, ParenthesizedExpression, FunctionCallExpression
};

use crate::parser::utils::{ handle_parse_error, handle_parse_error_for_option };
use crate::parser::expression_parser::parse_expression;


pub fn parse_atomic(tokens: &mut Peekable<Iter<Token>>) -> Expression {
    let atom = match tokens.next() {
        Some(Token::Literal(literal)) => AtomicExpression::Literal(literal.clone()),

        Some(Token::OpenParen) => AtomicExpression::Parenthesized(
            parse_parenthesized(tokens)
        ),

        Some(Token::Identifier(identifier)) => {
            match tokens.peek() {
                Some(Token::OpenParen) => AtomicExpression::FunctionCall(
                    parse_function_call(identifier, tokens)
                ),

                // TODO: Array indexing

                _ => AtomicExpression::Identifier(identifier.clone()),
            }
        }

        // TODO: Array literals

        token => handle_parse_error_for_option("Expected an atomic expression.", token),
    };
    Expression::Atomic(atom)
}


fn parse_parenthesized(tokens: &mut Peekable<Iter<Token>>) -> ParenthesizedExpression {
    let expression = parse_expression(tokens);

    match tokens.peek() {
        Some(Token::CloseParen) => {
            tokens.next()
        },
        _ => handle_parse_error_for_option("Expected closing parenthesis", tokens.peek()),
    };
    ParenthesizedExpression{ value: Box::new(expression) }
}


fn parse_function_call(identifier: &Identifier, tokens: &mut Peekable<Iter<Token>>) -> FunctionCallExpression {
    let parameters = parse_parameter_list(tokens);

    FunctionCallExpression {
        name: identifier.clone(),
        parameters,
    }
}


fn parse_parameter_list(tokens: &mut Peekable<Iter<Token>>) -> Vec<Expression> {
    if let Some(token) = tokens.next() {
        match token {
            Token::OpenParen => {},
            _ => handle_parse_error("Expected a parameter list starting with an open parenthesis", token),
        }
    }

    let mut parameters = vec![];

    while let Some(token) = tokens.peek() {
        match token {
            Token::Newline => {tokens.next();},
            Token::ListSeparator => {
                tokens.next();
                if let Some(Token::ListSeparator) | Some(Token::CloseParen) = tokens.peek() {
                    handle_parse_error_for_option::<()>("Expected a parameter", tokens.peek());
                }
            },
            Token::CloseParen => {
                tokens.next();
                break;
            }
            _ => parameters.push(parse_expression(tokens)),
        }
    }
    parameters
}


#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn test_parse_atomic_literal() {
        let tokens = vec![
            Token::Literal(Literal::Integer(1)),
            Token::Newline,
            Token::Literal(Literal::String("This is the next expression".to_string())),
        ];
        let iter_tokens = &mut tokens.iter().peekable();

        let expected = Expression::Atomic(
            AtomicExpression::Literal(Literal::Integer(1))
        );

        assert_eq!(parse_atomic(iter_tokens), expected);
        assert_eq!(Token::Newline, *iter_tokens.next().unwrap());
    }

    #[test]
    fn test_parse_atomic_identifier() {
        let tokens = vec![
            Token::Identifier(Identifier::Simple("identifier".to_string())),
            Token::Operator(Operator::Plus),
            Token::Literal(Literal::String("This is the next expression".to_string()))
        ];
        let iter_tokens = &mut tokens.iter().peekable();

        let expected = Expression::Atomic(
            AtomicExpression::Identifier(Identifier::Simple("identifier".to_string()))
        );

        assert_eq!(parse_atomic(iter_tokens), expected);
        assert_eq!(Token::Operator(Operator::Plus), *iter_tokens.next().unwrap());
    }

}