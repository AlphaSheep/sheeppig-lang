use std::iter::Peekable;
use std::slice::Iter;

use crate::elements::{Identifier, Operator, Literal, Keyword};
use crate::tokens::Token;


pub fn preprocess(input: &[Token]) -> Vec<Token> {
    let mut tokens = input.iter().peekable();

    let mut output: Vec<Token> = vec![];

    while let Some(token) = tokens.next() {
        match token {

            // Skip redundant newlines
            Token::Newline => if output.last() == Some(&Token::Newline) {
                continue;
            },

            // Newline after opening brackets is redundant
            Token::OpenParen | Token::OpenBrace | Token::OpenSquareBracket
            => if let Some(Token::Newline) = tokens.peek() {
                tokens.next();
            },

            // Newline after a list separator is redundant
            Token::ListSeparator => if let Some(Token::Newline) = tokens.peek() {
                tokens.next();
            },

            // Combine compound identifiers
            Token::Identifier(_) => if let Some(Token::Dot) = tokens.peek() {
                let new_token = combine_compound_identifier(token, &mut tokens);
                output.push(new_token);
                continue;  // We can skip to the next token, since we don't want to push the old identifier
            }

            _ => (),
        }
        output.push(token.clone());
    }
    output
}


fn combine_compound_identifier(token: &Token, tokens: &mut Peekable<Iter<Token>>) -> Token {
    let current_identifier = match token {
        Token::Identifier(identifier) => identifier,
        _ => panic!("Token must be Token::Identifier, found {:?}", token),
    };

    let mut identifiers = vec![current_identifier.as_string()];

    while let Some(Token::Dot) = tokens.peek() {
        tokens.next();  // Consume the dot
        match tokens.next() {
            Some(Token::Identifier(identifier)) => identifiers.push(identifier.as_string()),
            _ => panic!("Expected identifier after dot, found {:?}", tokens.peek()),
        }
    }

    Token::Identifier(Identifier::Compound(identifiers))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_redundant_newlines() {
        let input = vec![Token::Newline, Token::Newline, Token::Newline];
        let expected = vec![Token::Newline];

        assert_eq!(preprocess(&input), expected);
    }

    #[test]
    fn test_remove_newline_after_open_brace() {
        let input = vec![Token::OpenBrace, Token::Newline];
        let expected = vec![Token::OpenBrace];

        assert_eq!(preprocess(&input), expected);
    }

    #[test]
    fn test_remove_newline_after_list_separator() {
        let input = vec![Token::ListSeparator, Token::Newline];
        let expected = vec![Token::ListSeparator];

        assert_eq!(preprocess(&input), expected);
    }

    #[test]
    fn test_combine_leaves_simple_identifier() {
        let input = vec![Token::Identifier(Identifier::Simple("foo".to_string())), Token::Assign];
        let expected = vec![Token::Identifier(Identifier::Simple("foo".to_string())), Token::Assign];

        assert_eq!(preprocess(&input), expected);
    }

    #[test]
    fn test_combine_compound_identifier() {
        let input = vec![Token::Identifier(Identifier::Simple("foo".to_string())), Token::Dot, Token::Identifier(Identifier::Simple("bar".to_string())), Token::Assign];
        let expected = vec![Token::Identifier(Identifier::Compound(vec!["foo".to_string(), "bar".to_string()])), Token::Assign];

        assert_eq!(preprocess(&input), expected);
    }
}