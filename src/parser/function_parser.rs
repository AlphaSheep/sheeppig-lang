use std::iter::Peekable;
use std::slice::Iter;

use crate::elements::Identifier;
use crate::tokens::Token;
use crate::tree;
use crate::parser::statement_parser::parse_statement_block_between_braces;

use crate::parser::utils::{handle_parse_error, handle_parse_error_for_option};


pub fn parse_function_block(tokens: &mut Peekable<Iter<Token>>) -> tree::Function {
    let name = parse_function_name(tokens);
    let parameters = parse_parameter_list(tokens);
    let return_type = parse_function_return_type(tokens);
    let body = parse_statement_block_between_braces(tokens);

    tree::Function {
        name,
        parameters,
        return_type,
        body: Box::new(body),
    }
}


fn parse_function_name(tokens: &mut Peekable<Iter<Token>>) -> Identifier {
    match tokens.next() {
        Some(Token::Identifier(identifier)) => identifier.clone(),
        _ => handle_parse_error_for_option("Expected identifier after function keyword", tokens.peek()),
    }
}


fn parse_parameter_list(tokens: &mut Peekable<Iter<Token>>) -> Vec<tree::Parameter> {
    if let Some(token) = tokens.next() {
        match token {
            Token::OpenParen => {},
            _ => handle_parse_error("Expected a parameter list starting with an open parenthesis", token),
        }
    }

    let mut parameters = vec![];

    while let Some(token) = tokens.next() {
        match token {
            Token::Newline => continue,
            Token::ListSeparator => {
                if let Some(Token::ListSeparator) = tokens.peek() {
                    handle_parse_error_for_option::<()>("Expected a parameter", tokens.peek());
                }
            },
            Token::CloseParen => break,
            Token::Identifier(_) => {
                parameters.push(parse_parameter(token, tokens))
            },
            _ => handle_parse_error("Expected a parameter or a closing parenthesis", token),
        }
    }

    parameters
}


fn parse_parameter(current: &Token, tokens: &mut Peekable<Iter<Token>>) -> tree::Parameter {
    let name = match current {
        Token::Identifier(identifier) => identifier.clone(),
        _ => handle_parse_error("Expected an identifier", current),
    };

    match tokens.peek() {
        Some(Token::Colon) => { tokens.next(); }
        _ => handle_parse_error_for_option("Expected colon after parameter name", tokens.peek()),
    }

    let param_type = match tokens.next() {
        Some(Token::Identifier(identifier)) => identifier.clone(),
        _ => handle_parse_error_for_option("Expected a type identifier after colon", tokens.peek()),
    };

    tree::Parameter {
        name,
        param_type,
    }
}


fn parse_function_return_type(tokens: &mut Peekable<Iter<Token>>) -> Option<Identifier> {
    if let Some(Token::Colon) = tokens.peek() {
        tokens.next();  // Consume the colon
        match tokens.next() {
            Some(Token::Identifier(identifier)) => Some(identifier.clone()),
            _ => handle_parse_error_for_option("Expected type identifier after function parameters", tokens.peek()),
        }
    } else {
        None
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_function_name() {
        let tokens = vec![
            Token::Identifier(Identifier::Simple("foo".to_string())),
        ];

        let expected = Identifier::Simple("foo".to_string());

        assert_eq!(parse_function_name(&mut tokens.iter().peekable()), expected);
    }

    #[test]
    fn test_parse_parameter_list_single_parameter() {
        let tokens = vec![
            Token::OpenParen,
            Token::Identifier(Identifier::Simple("x".to_string())),
            Token::Colon,
            Token::Identifier(Identifier::Simple("int".to_string())),
            Token::CloseParen,
        ];

        let expected = vec![
            tree::Parameter {
                name: Identifier::Simple("x".to_string()),
                param_type: Identifier::Simple("int".to_string()),
            }
        ];

        assert_eq!(parse_parameter_list(&mut tokens.iter().peekable()), expected);
    }

    #[test]
    fn test_parse_parameter_list_two_parameters() {
        let tokens = vec![
            Token::OpenParen,
            Token::Identifier(Identifier::Simple("x".to_string())),
            Token::Colon,
            Token::Identifier(Identifier::Simple("int".to_string())),
            Token::ListSeparator,
            Token::Identifier(Identifier::Simple("y".to_string())),
            Token::Colon,
            Token::Identifier(Identifier::Simple("int".to_string())),
            Token::CloseParen,
        ];

        let expected = vec![
            tree::Parameter {
                name: Identifier::Simple("x".to_string()),
                param_type: Identifier::Simple("int".to_string()),
            },
            tree::Parameter {
                name: Identifier::Simple("y".to_string()),
                param_type: Identifier::Simple("int".to_string()),
            }
        ];

        assert_eq!(parse_parameter_list(&mut tokens.iter().peekable()), expected);
    }

    #[test]
    fn test_parse_parameter() {
        let current = Token::Identifier(Identifier::Simple("x".to_string()));
        let tokens = vec![
            Token::Colon,
            Token::Identifier(Identifier::Simple("int".to_string())),
        ];

        let expected = tree::Parameter {
            name: Identifier::Simple("x".to_string()),
            param_type: Identifier::Simple("int".to_string()),
        };

        assert_eq!(parse_parameter(&current, &mut tokens.iter().peekable()), expected);
    }

    #[test]
    fn test_parse_function_return_type() {
        let tokens = vec![
            Token::Colon,
            Token::Identifier(Identifier::Simple("int".to_string())),
        ];

        let expected = Some(Identifier::Simple("int".to_string()));

        assert_eq!(parse_function_return_type(&mut tokens.iter().peekable()), expected);
    }

    #[test]
    fn test_parse_function_no_return_type() {
        let tokens = vec![
            Token::OpenBrace,
            Token::Newline,
        ];
        let iter_tokens = &mut tokens.iter().peekable();

        assert_eq!(parse_function_return_type(iter_tokens), None);
        assert_eq!(Token::OpenBrace, *iter_tokens.next().unwrap());
    }

    #[test]
    fn test_parse_function_block() {
        let tokens = vec![
            Token::Identifier(Identifier::Simple("foo".to_string())),
            Token::OpenParen,
            Token::Identifier(Identifier::Simple("x".to_string())),
            Token::Colon,
            Token::Identifier(Identifier::Simple("int".to_string())),
            Token::CloseParen,
            Token::Colon,
            Token::Identifier(Identifier::Simple("float".to_string())),
            Token::OpenBrace,
            Token::CloseBrace,
        ];

        let expected = tree::Function {
            name: Identifier::Simple("foo".to_string()),
            parameters: vec![
                tree::Parameter {
                    name: Identifier::Simple("x".to_string()),
                    param_type: Identifier::Simple("int".to_string()),
                }
            ],
            return_type: Some(Identifier::Simple("float".to_string())),
            body: Box::new(tree::StatementBlock {
                statements: vec![],
            }),
        };

        assert_eq!(parse_function_block(&mut tokens.iter().peekable()), expected);
    }

}
