use std::str::Chars;
use std::iter::Peekable;

use crate::elements::{Identifier, Literal, Operator, Keyword};
use crate::tokens::Token;


pub fn tokenize(source_code: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = source_code.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::CloseParen),
            '{' => tokens.push(Token::OpenBrace),
            '}' => tokens.push(Token::CloseBrace),
            '[' => tokens.push(Token::OpenBracket),
            ']' => tokens.push(Token::CloseBracket),
            ',' => tokens.push(Token::ListSeparator),
            ':' => tokens.push(Token::Colon),

            '.' => {
                if let Some('0'..='9') = chars.peek() {
                    read_number_literal(c, &mut chars, &mut tokens);
                } else {
                    tokens.push(Token::Dot);
                }
            },

            '+' => tokens.push(Token::Operator(Operator::Plus)),
            '-' => tokens.push(Token::Operator(Operator::Minus)),
            '*' => {
                if let Some('*') = chars.peek() {
                    chars.next();
                    tokens.push(Token::Operator(Operator::Power));
                } else {
                    tokens.push(Token::Operator(Operator::Times));
                }
            },
            '/' => {
                if let Some('*') = chars.peek() {
                    eat_block_comment(&mut chars);
                } else {
                    tokens.push(Token::Operator(Operator::Divide));
                }
            },
            '\\' => {
                if let Some('\n') | Some('\r') = chars.peek() {
                    eat_whitespace('\\', &mut chars, &mut tokens, false)
                } else {
                    panic!("Unexpected character: {}", c);
                }
            },
            '%' => tokens.push(Token::Operator(Operator::Modulo)),

            '&' => {
                if let Some('&') = chars.peek() {
                    chars.next();
                    tokens.push(Token::Operator(Operator::And));
                } else {
                    tokens.push(Token::Operator(Operator::BitwiseAnd));
                }
            },

            '|' => {
                if let Some('|') = chars.peek() {
                    chars.next();
                    tokens.push(Token::Operator(Operator::Or));
                } else {
                    tokens.push(Token::Operator(Operator::BitwiseOr));
                }
            },

            '^' => tokens.push(Token::Operator(Operator::BitwiseXor)),
            '~' => tokens.push(Token::Operator(Operator::BitwiseNot)),
            '!' => {
                if let Some('=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::Operator(Operator::NotEqual));
                } else {
                    tokens.push(Token::Operator(Operator::Not));
                }
            }

            '<' => {
                if let Some('<') = chars.peek() {
                    chars.next();
                    tokens.push(Token::Operator(Operator::BitwiseLeftShift));
                } else if let Some('=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::Operator(Operator::LessThanOrEqual));
                } else {
                    tokens.push(Token::Operator(Operator::LessThan));
                }
            },
            '>' => {
                if let Some('>') = chars.peek() {
                    chars.next();
                    tokens.push(Token::Operator(Operator::BitwiseRightShift));
                } else if let Some('=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::Operator(Operator::GreaterThanOrEqual));
                } else {
                    tokens.push(Token::Operator(Operator::GreaterThan));
                }
            },
            '=' => {
                if let Some('=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::Operator(Operator::Equal));
                } else {
                    tokens.push(Token::Assign);
                }
            },
            '?' => tokens.push(Token::TernaryCondition),

            '\'' => read_char_literal(&mut chars, &mut tokens),
            '"' => read_string_literal(&mut chars, &mut tokens),
            '0'..='9' => read_number_literal(c, &mut chars, &mut tokens),
            'a'..='z' | 'A'..='Z' | '_' => read_alphanumeric_sequence(c, &mut chars, &mut tokens),

            ' ' | '\t' | '\n' | '\r' => eat_whitespace(c, &mut chars, &mut tokens, true),
            '#' => eat_inline_comment(&mut chars, &mut tokens),

            _ => panic!("Unexpected character: {}", c),
        }
    }

    tokens.push(Token::EndOfModule);
    tokens
}


fn read_char_literal(chars: &mut Peekable<Chars>, tokens: &mut Vec<Token>) {
    let char = chars.next();
    match char {
        Some('\'') => panic!("Empty character literal"),
        Some('\\') => {
            let escaped_char = convert_escaped_char(chars.next());
            tokens.push(Token::Literal(Literal::Char(escaped_char)));
        },
        Some(c) => tokens.push(Token::Literal(Literal::Char(c))),
        None => panic!("Unexpected end of file"),
    }

    if chars.next() != Some('\'') {
        panic!("Character literal must contain only one character");
    }
}

fn read_string_literal(chars: &mut Peekable<Chars>, tokens: &mut Vec<Token>) {
    let mut string = String::new();

    while let Some(c) = chars.next() {
        match c {
            '\\' => string.push(convert_escaped_char(chars.next())),
            '"' => break,
            _ => string.push(c),
        }
    }

    tokens.push(Token::Literal(Literal::String(string)));
}


fn convert_escaped_char(char: Option<char>) -> char {
    return match char {
        Some('n') => '\n',
        Some('r') => '\r',
        Some('t') => '\t',
        Some('\'') => '\'',
        Some('"') => '"',
        Some('\\') => '\\',
        Some('0') => '\0',
        Some(_) => panic!("Unrecognised escape sequence"),
        None => panic!("Unexpected end of file"),
    }
}


fn read_number_literal(current: char, chars: &mut Peekable<Chars>, tokens: &mut Vec<Token>) {
    let mut number = String::new();
    number.push(current);

    let mut is_float = current == '.';
    let mut is_exponent = false;

    while let Some(c) = chars.peek() {
        match c {
            '0'..='9' => number.push(*c),
            '_' => (),
            '.' => {
                if is_float {
                    panic!("Unexpected extra decimal point in number literal");
                } else {
                    is_float = true;
                    number.push(*c);
                }
            },
            'E' | 'e' => {
                is_float = true;
                is_exponent = true;
                number.push(*c);
                chars.next();
                break;
            }
            _ => break,
        }
        chars.next();
    }

    if is_exponent {
        read_exponent(chars, &mut number);
    }

    if is_float {
        tokens.push(Token::Literal(Literal::Float(number.parse().unwrap())));
    } else {
        tokens.push(Token::Literal(Literal::Integer(number.parse().unwrap())));
    }
}


fn read_exponent(chars: &mut Peekable<Chars>, number: &mut String) {
    if let Some(c) = chars.peek() {
        match c {
            '+' | '-' => {
                number.push(*c);
                chars.next();
            },
            _ => (),
        }
    }
    while let Some(c) = chars.peek() {
        match c {
            '0'..='9' => number.push(*c),
            '_' => (),
            _ => break,
        }
        chars.next();
    }
}


fn read_alphanumeric_sequence(current: char, chars: &mut Peekable<Chars>, tokens: &mut Vec<Token>) {
    let mut identifier = String::new();
    identifier.push(current);

    while let Some(c) = chars.peek() {
        match c {
            'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => identifier.push(*c),
            _ => break,
        }
        chars.next();
    }

    tokens.push(match_keyword_or_literal(&identifier));
}


fn match_keyword_or_literal(identifier: &str) -> Token {
    match Keyword::from_str(identifier) {
        Some(keyword) => Token::Keyword(keyword),
        None => match Literal::from_str(identifier) {
            Some(literal) => Token::Literal(literal),
            None => Token::Identifier(Identifier::Simple(
                identifier.to_string()
            )),
        }
    }
}


fn eat_whitespace(current: char, chars: &mut Peekable<Chars>, tokens: &mut Vec<Token>, allow_newline: bool) {
    let mut is_newline = current == '\n' || current == '\r';
    while let Some(c) = chars.peek() {
        match c {
            ' ' | '\t' => { chars.next(); },
            '\n' | '\r' => {
                is_newline = true;
                chars.next();
            },
            '#' => {
                eat_inline_comment(chars, tokens);
            },
            _ => break,
        };
    }
    if allow_newline && is_newline && tokens.last() != Some(&Token::Newline) {
        tokens.push(Token::Newline);
    }
}


fn eat_inline_comment(chars: &mut Peekable<Chars>, tokens: &mut Vec<Token>) {
    while let Some(c) = chars.peek() {
        match c {
            '\n' | '\r' => break,
            _ => chars.next(),
        };
    }
}


fn eat_block_comment(chars: &mut Peekable<Chars>) {
    while let Some(c) = chars.next() {
        match c {
            '*' => {
                if let Some('/') = chars.peek() {
                    chars.next();
                    break;
                }
            },
            _ => (),
        }
    }
}


#[cfg(test)]
mod test {
    use crate::tokens;

    use super::*;

    #[test]
    fn test_read_valid_alphanumeric_sequence() {
        let mut chars = "read_this but don't read this".chars().peekable();
        let mut tokens = Vec::new();

        read_alphanumeric_sequence('_', &mut chars, &mut tokens);

        assert_eq!(tokens, vec![Token::Identifier(Identifier::Simple("_read_this".to_string()))]);
        assert_eq!(chars.next(), Some(' '));
    }

    #[test]
    fn test_read_keyword() {
        let mut chars = "un name(params)".chars().peekable();
        let mut tokens = Vec::new();

        read_alphanumeric_sequence('f', &mut chars, &mut tokens);

        assert_eq!(tokens, vec![Token::Keyword(Keyword::Function)]);
        assert_eq!(chars.next(), Some(' '))
    }

    #[test]
    fn test_read_identifier_starting_with_keyword() {
        let mut chars = "un_name".chars().peekable();
        let mut tokens = Vec::new();

        read_alphanumeric_sequence('f', &mut chars, &mut tokens);

        assert_eq!(tokens, vec![Token::Identifier(Identifier::Simple("fun_name".to_string()))]);
    }

    #[test]
    fn test_read_literal() {
        let mut chars = "alse".chars().peekable();
        let mut tokens = Vec::new();

        read_alphanumeric_sequence('f', &mut chars, &mut tokens);

        assert_eq!(tokens, vec![Token::Literal(Literal::Boolean(false))]);
    }

    #[test]
    fn test_read_many_digit_integer() {
        let mut chars = "234+3".chars().peekable();
        let mut tokens = Vec::new();

        read_number_literal('1', &mut chars, &mut tokens);

        assert_eq!(tokens, vec![Token::Literal(Literal::Integer(1234))]);
        assert_eq!(chars.next(), Some('+'));
    }

    #[test]
    fn test_read_single_digit() {
        let mut chars = " but this is not an integer".chars().peekable();
        let mut tokens = Vec::new();

        read_number_literal('1', &mut chars, &mut tokens);

        assert_eq!(tokens, vec![Token::Literal(Literal::Integer(1))]);
        assert_eq!(chars.next(), Some(' '));
    }

    #[test]
    fn test_read_integer_with_underscores() {
        let mut chars = "23_456_789".chars().peekable();
        let mut tokens = Vec::new();

        read_number_literal('1', &mut chars, &mut tokens);

        assert_eq!(tokens, vec![Token::Literal(Literal::Integer(123456789))]);
    }

    #[test]
    fn test_read_float() {
        let mut chars = ".141592".chars().peekable();
        let mut tokens = Vec::new();

        read_number_literal('3', &mut chars, &mut tokens);

        assert_eq!(tokens, vec![Token::Literal(Literal::Float(3.141592))]);
    }

    #[test]
    fn test_read_bigger_float() {
        let mut chars = "234.5678".chars().peekable();
        let mut tokens = Vec::new();

        read_number_literal('1', &mut chars, &mut tokens);

        assert_eq!(tokens, vec![Token::Literal(Literal::Float(1234.5678))]);
    }

    #[test]
    fn test_read_integer_as_a_float() {
        let mut chars = "234. something else".chars().peekable();
        let mut tokens = Vec::new();

        read_number_literal('1', &mut chars, &mut tokens);

        assert_eq!(tokens, vec![Token::Literal(Literal::Float(1234.0))]);
        assert_eq!(chars.next(), Some(' '));
    }

    #[test]
    fn test_read_scientific_notation_big() {
        let mut chars = ".2345E+67 and some more".chars().peekable();
        let mut tokens = Vec::new();

        read_number_literal('1', &mut chars, &mut tokens);

        assert_eq!(tokens, vec![Token::Literal(Literal::Float(1.2345E+67))]);
        assert_eq!(chars.next(), Some(' '));
    }

    #[test]
    fn test_read_scientific_notation_tiny() {
        let mut chars = ".2345e-67".chars().peekable();
        let mut tokens = Vec::new();

        read_number_literal('1', &mut chars, &mut tokens);

        assert_eq!(tokens, vec![Token::Literal(Literal::Float(1.2345E-67))]);
    }

    #[test]
    fn test_read_scientific_notation_no_symbol() {
        let mut chars = ".2345e67".chars().peekable();
        let mut tokens = Vec::new();

        read_number_literal('1', &mut chars, &mut tokens);

        assert_eq!(tokens, vec![Token::Literal(Literal::Float(1.2345E+67))]);
    }

    #[test]
    fn test_read_char_literal() {
        let mut chars = "a'".chars().peekable();
        let mut tokens = Vec::new();

        read_char_literal(&mut chars, &mut tokens);

        assert_eq!(tokens, vec![Token::Literal(Literal::Char('a'))]);
    }

    #[test]
    fn test_read_escaped_char() {
        let mut chars = "\\n'".chars().peekable();
        let mut tokens = Vec::new();

        read_char_literal(&mut chars, &mut tokens);

        assert_eq!(tokens, vec![Token::Literal(Literal::Char('\n'))]);
    }

    #[test]
    fn test_convert_escaped_char() {
        assert_eq!(convert_escaped_char(Some('n')), '\n');
        assert_eq!(convert_escaped_char(Some('r')), '\r');
        assert_eq!(convert_escaped_char(Some('t')), '\t');
        assert_eq!(convert_escaped_char(Some('\'')), '\'');
        assert_eq!(convert_escaped_char(Some('"')), '"');
        assert_eq!(convert_escaped_char(Some('\\')), '\\');
        assert_eq!(convert_escaped_char(Some('0')), '\0');
    }

    #[test]
    fn test_read_string_literal() {
        let mut chars = "this is a string\" but this is not a string".chars().peekable();
        let mut tokens = Vec::new();

        read_string_literal(&mut chars, &mut tokens);

        assert_eq!(tokens, vec![Token::Literal(Literal::String("this is a string".to_string()))]);
    }

    #[test]
    fn test_read_string_literal_with_escaped_doublequote() {
        let mut chars = "this is a string with a \\\" in it\" but this is not a string".chars().peekable();
        let mut tokens = Vec::new();

        read_string_literal(&mut chars, &mut tokens);

        assert_eq!(tokens, vec![Token::Literal(Literal::String("this is a string with a \" in it".to_string()))]);
    }

    #[test]
    fn test_eat_whitespace() {
        let mut chars = "      \tHello?".chars().peekable();
        let mut tokens = Vec::new();

        eat_whitespace(' ', &mut chars, &mut tokens, true);

        assert_eq!(tokens, vec![]);
        assert_eq!(chars.next(), Some('H'));
    }

    #[test]
    fn test_eat_whitespace_with_one_newline() {
        let mut chars = "      \nHello?".chars().peekable();
        let mut tokens = Vec::new();

        eat_whitespace(' ', &mut chars, &mut tokens, true);

        assert_eq!(tokens, vec![Token::Newline]);
        assert_eq!(chars.next(), Some('H'));
    }

    #[test]
    fn test_eat_whitespace_with_two_newlines() {
        let mut chars = "      \n\nHello?".chars().peekable();
        let mut tokens = Vec::new();

        eat_whitespace(' ', &mut chars, &mut tokens, true);

        assert_eq!(tokens, vec![Token::Newline]);
        assert_eq!(chars.next(), Some('H'));
    }

    #[test]
    fn test_eat_whitspace_with_just_newline() {
        let mut chars = "Hello?".chars().peekable();
        let mut tokens = Vec::new();

        eat_whitespace('\n', &mut chars, &mut tokens, true);

        assert_eq!(tokens, vec![Token::Newline]);
        assert_eq!(chars.next(), Some('H'));
    }

    #[test]
    fn test_eat_whitespace_with_newline_not_allowed() {
        let mut chars = "      \nHello?".chars().peekable();
        let mut tokens = Vec::new();

        eat_whitespace(' ', &mut chars, &mut tokens, false);

        assert_eq!(tokens, vec![]);
        assert_eq!(chars.next(), Some('H'));
    }

    #[test]
    fn test_eat_inline_comment() {
        let mut chars = "this is a comment\nBut this is not".chars().peekable();
        let mut tokens = Vec::new();

        eat_inline_comment(&mut chars, &mut tokens);

        assert_eq!(tokens, vec![]);
        assert_eq!(chars.next(), Some('\n'));
        assert_eq!(chars.next(), Some('B'));
    }

    #[test]
    fn test_eat_block_comment() {
        let mut chars = "*this is a comment */But this is not".chars().peekable();

        eat_block_comment(&mut chars);

        assert_eq!(chars.next(), Some('B'));
    }

    #[test]
    fn test_eat_whitespace_with_an_inline_comment() {
        let mut chars = "      # this is a comment\n     Hello?".chars().peekable();
        let mut tokens = Vec::new();

        eat_whitespace(' ', &mut chars, &mut tokens, true);

        assert_eq!(tokens, vec![Token::Newline]);
        assert_eq!(chars.next(), Some('H'));
    }

    #[test]
    fn test_eat_whitespace_with_a_block_comment() {
        let mut chars = "      /* this is a comment */      Hello?".chars().peekable();
        let mut tokens = Vec::new();

        eat_whitespace(' ', &mut chars, &mut tokens, true);

        assert_eq!(tokens, vec![]);
        assert_eq!(chars.next(), Some('/'));
    }
}
