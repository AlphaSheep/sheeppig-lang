use core::panic;
use std::iter::Peekable;
use std::slice::Iter;

use crate::elements::{ Identifier, Literal, Operator, Keyword };
use crate::tokens::Token;
use crate::tree::{
    Expression, AtomicExpression, ParenthesizedExpression, FunctionCallExpression
};

use crate::parser::utils::{ handle_parse_error, handle_parse_error_for_option };


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

        token => handle_parse_error_for_option("Expected a literal", token),
    };
    Expression::Atomic(atom)
}


fn parse_parenthesized(tokens: &mut Peekable<Iter<Token>>) -> ParenthesizedExpression {
    panic!("Not implemented");
}


fn parse_function_call(identifier: &Identifier, tokens: &mut Peekable<Iter<Token>>) -> FunctionCallExpression {
    panic!("Not implemented");
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