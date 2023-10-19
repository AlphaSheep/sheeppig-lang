use std::fs::read_to_string;

use sheeppig::elements::{Identifier, Literal, Operator, Keyword};
use sheeppig::tokens::Token;
use sheeppig::lexer::tokenize;


fn read_file(file_path: &str) -> String {
    let input = read_to_string(file_path)
        .expect("Failed to read input file");
    input
}


#[test]
fn test_tokenise_hello_world() {
    let source_code = read_file("./samples/test_samples/hello_world.sp");

    let tokens = tokenize(&source_code);

    let expected = vec![
        Token::Keyword(Keyword::Function),
        Token::Identifier(Identifier::Simple("hello_world".to_string())),
        Token::OpenParen,
        Token::CloseParen,
        Token::OpenBrace,

        Token::Identifier(Identifier::Simple("print".to_string())),
        Token::OpenParen,
        Token::Literal(Literal::String("Hello, world!".to_string())),
        Token::CloseParen,
        Token::Newline,

        Token::CloseBrace,
        Token::EndOfModule,
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_tokenise_adding() {
    let source_code = read_file("./samples/test_samples/adding.sp");
    let tokens = tokenize(&source_code);

    let expected = vec![
        Token::Keyword(Keyword::Function),
        Token::Identifier(Identifier::Simple("add".to_string())),
        Token::OpenParen,
        Token::Identifier(Identifier::Simple("a".to_string())),
        Token::Colon,
        Token::Identifier(Identifier::Simple("int".to_string())),
        Token::ListSeparator,
        Token::Identifier(Identifier::Simple("b".to_string())),
        Token::Colon,
        Token::Identifier(Identifier::Simple("int".to_string())),
        Token::CloseParen,
        Token::Colon,
        Token::Identifier(Identifier::Simple("int".to_string())),
        Token::OpenBrace,

        Token::Keyword(Keyword::Return),
        Token::Identifier(Identifier::Simple("a".to_string())),
        Token::Operator(Operator::Plus),
        Token::Identifier(Identifier::Simple("b".to_string())),
        Token::Newline,

        Token::CloseBrace,
        Token::EndOfModule,
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_tokenise_conditional() {
    let source_code = read_file("./samples/test_samples/conditional.sp");
    let tokens = tokenize(&source_code);

    let expected = vec![
        Token::Keyword(Keyword::Function),
        Token::Identifier(Identifier::Simple("main".to_string())),
        Token::OpenParen,
        Token::CloseParen,
        Token::OpenBrace,

        Token::Keyword(Keyword::Variable),
        Token::Identifier(Identifier::Simple("a".to_string())),
        Token::Assign,
        Token::Literal(Literal::Integer(6)),
        Token::Newline,

        Token::Identifier(Identifier::Simple("b".to_string())),
        Token::Assign,
        Token::Literal(Literal::Char('b')),
        Token::Newline,

        Token::Keyword(Keyword::If),
        Token::Literal(Literal::Float(1.2)),
        Token::Operator(Operator::LessThanOrEqual),
        Token::Literal(Literal::Integer(3)),
        Token::OpenBrace,

        Token::Identifier(Identifier::Simple("a".to_string())),
        Token::Assign,
        Token::Identifier(Identifier::Simple("a".to_string())),
        Token::Operator(Operator::Times),
        Token::Literal(Literal::Integer(2)),
        Token::Newline,

        Token::CloseBrace,
        Token::Keyword(Keyword::Else),
        Token::OpenBrace,

        Token::Identifier(Identifier::Simple("a".to_string())),
        Token::Assign,
        Token::Literal(Literal::Integer(5)),
        Token::Operator(Operator::LessThan),
        Token::Literal(Literal::Integer(4)),
        Token::TernaryCondition,
        Token::Literal(Literal::Integer(6)),
        Token::Colon,
        Token::Literal(Literal::Integer(7)),
        Token::Newline,

        Token::CloseBrace,
        Token::Newline,

        Token::CloseBrace,
        Token::EndOfModule,
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_tokenise_import() {
    let source_code = read_file("./samples/test_samples/import.sp");
    let tokens = tokenize(&source_code);

    let expected = vec![
        Token::Keyword(Keyword::Using),
        Token::OpenBrace,

        Token::Identifier(Identifier::Simple("sqrt".to_string())),
        Token::Keyword(Keyword::As),
        Token::Identifier(Identifier::Simple("square_root".to_string())),
        Token::Keyword(Keyword::From),
        Token::Identifier(Identifier::Compound(vec![
            "math".to_string(), "utils".to_string()
        ])),
        Token::Newline,

        Token::Identifier(Identifier::Simple("sin".to_string())),
        Token::ListSeparator,
        Token::Identifier(Identifier::Simple("cos".to_string())),
        Token::Keyword(Keyword::From),
        Token::Identifier(Identifier::Compound(vec![
            "math".to_string(), "trig".to_string()
        ])),
        Token::Newline,

        Token::CloseBrace,
        Token::Newline,

        Token::Keyword(Keyword::Function),
        Token::Identifier(Identifier::Simple("main".to_string())),
        Token::OpenParen,
        Token::CloseParen,
        Token::OpenBrace,

        Token::CloseBrace,
        Token::EndOfModule,
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_tokenise_arrays_and_numbers() {
    let source_code = read_file("./samples/test_samples/arrays.sp");
    let tokens = tokenize(&source_code);

    let expected = vec![
        Token::Keyword(Keyword::Function),
        Token::Identifier(Identifier::Simple("array_stuff".to_string())),
        Token::OpenParen,
        Token::CloseParen,
        Token::OpenBrace,

        Token::Identifier(Identifier::Simple("array".to_string())),
        Token::Assign,
        Token::OpenSquareBracket,
        Token::Literal(Literal::Integer(1)),
        Token::ListSeparator,
        Token::Literal(Literal::Integer(23)),
        Token::ListSeparator,
        Token::Literal(Literal::Float(4.5)),
        Token::ListSeparator,
        Token::Literal(Literal::Float(0.0123)),
        Token::ListSeparator,
        Token::Literal(Literal::Integer(6789012)),
        Token::ListSeparator,
        Token::Literal(Literal::Float(1234.56789)),
        Token::ListSeparator,
        Token::Literal(Literal::Float(1000000.0)),
        Token::ListSeparator,
        Token::Literal(Literal::Float(0.000001)),
        Token::ListSeparator,
        Token::Literal(Literal::Float(1340000.0)),
        Token::CloseSquareBracket,
        Token::Newline,

        Token::CloseBrace,
        Token::EndOfModule,
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_tokenise_arithmetic() {
    let source_code = read_file("./samples/test_samples/arithmetic.sp");
    let tokens = tokenize(&source_code);

    let expected = vec![
        Token::Keyword(Keyword::Function),
        Token::Identifier(Identifier::Simple("math_fun".to_string())),
        Token::OpenParen,
        Token::CloseParen,
        Token::OpenBrace,

        Token::Identifier(Identifier::Simple("a".to_string())),
        Token::Assign,
        Token::OpenParen,
        Token::Literal(Literal::Integer(1)),
        Token::Operator(Operator::Plus),
        Token::Literal(Literal::Integer(2)),
        Token::Operator(Operator::Minus),
        Token::Literal(Literal::Integer(3)),
        Token::CloseParen,
        Token::Operator(Operator::Divide),
        Token::Literal(Literal::Integer(4)),
        Token::Operator(Operator::Times),
        Token::Literal(Literal::Integer(5)),
        Token::Operator(Operator::Modulo),
        Token::Literal(Literal::Integer(6)),
        Token::Operator(Operator::Power),
        Token::Literal(Literal::Integer(7)),
        Token::Newline,

        Token::Identifier(Identifier::Simple("b".to_string())),
        Token::Operator(Operator::Plus),
        Token::Assign,
        Token::Literal(Literal::Integer(7)),
        Token::Newline,

        Token::Identifier(Identifier::Simple("long_expression".to_string())),
        Token::Assign,
        Token::Literal(Literal::Integer(123)),
        Token::Operator(Operator::Minus),
        Token::Literal(Literal::Integer(456)),
        Token::Operator(Operator::Plus),
        Token::Literal(Literal::Integer(789)),
        Token::Operator(Operator::Times),
        Token::Literal(Literal::Integer(123)),
        Token::Operator(Operator::Divide),
        Token::Literal(Literal::Integer(456)),
        Token::Operator(Operator::Modulo),
        Token::Literal(Literal::Integer(789)),
        Token::Newline,

        Token::Identifier(Identifier::Simple("logic".to_string())),
        Token::Assign,
        Token::Literal(Literal::Integer(1)),
        Token::Operator(Operator::BitwiseAnd),
        Token::Literal(Literal::Integer(2)),
        Token::Operator(Operator::BitwiseOr),
        Token::Literal(Literal::Integer(3)),
        Token::Operator(Operator::BitwiseXor),
        Token::Literal(Literal::Integer(4)),
        Token::Operator(Operator::BitwiseLeftShift),
        Token::Literal(Literal::Integer(5)),
        Token::Operator(Operator::BitwiseRightShift),
        Token::Literal(Literal::Integer(6)),
        Token::Newline,

        Token::Identifier(Identifier::Simple("bitwise_not".to_string())),
        Token::Assign,
        Token::Operator(Operator::BitwiseNot),
        Token::Literal(Literal::Integer(5)),
        Token::Newline,

        Token::Identifier(Identifier::Simple("negative".to_string())),
        Token::Assign,
        Token::Operator(Operator::Minus),
        Token::Literal(Literal::Integer(5)),
        Token::Newline,

        Token::Identifier(Identifier::Simple("compare".to_string())),
        Token::Assign,
        Token::OpenParen,
        Token::Literal(Literal::Integer(1)),
        Token::Operator(Operator::LessThan),
        Token::Literal(Literal::Integer(2)),
        Token::CloseParen,
        Token::Operator(Operator::And),
        Token::OpenParen,
        Token::Literal(Literal::Integer(3)),
        Token::Operator(Operator::GreaterThan),
        Token::Literal(Literal::Integer(4)),
        Token::CloseParen,
        Token::Operator(Operator::Or),
        Token::Operator(Operator::Not),
        Token::OpenParen,
        Token::Literal(Literal::Integer(5)),
        Token::Operator(Operator::LessThanOrEqual),
        Token::Literal(Literal::Integer(6)),
        Token::CloseParen,
        Token::Operator(Operator::Or),
        Token::Literal(Literal::Integer(7)),
        Token::Operator(Operator::Equal),
        Token::Literal(Literal::Integer(8)),
        Token::Operator(Operator::Or),
        Token::Literal(Literal::Integer(9)),
        Token::Operator(Operator::GreaterThanOrEqual),
        Token::Literal(Literal::Integer(10)),
        Token::Operator(Operator::And),
        Token::Literal(Literal::Integer(5)),
        Token::Operator(Operator::NotEqual),
        Token::Literal(Literal::Integer(6)),
        Token::Newline,

        Token::CloseBrace,
        Token::EndOfModule,
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_tokenise_comments() {
    let source_code = read_file("./samples/test_samples/comments.sp");
    let tokens = tokenize(&source_code);

    let expected = vec![
        Token::Keyword(Keyword::Function),
        Token::Identifier(Identifier::Simple("main".to_string())),
        Token::OpenParen,
        Token::CloseParen,
        Token::OpenBrace,

        Token::Identifier(Identifier::Simple("a".to_string())),
        Token::Assign,
        Token::Literal(Literal::Integer(1)),
        Token::Newline,

        Token::Identifier(Identifier::Simple("b".to_string())),
        Token::Assign,
        Token::Literal(Literal::Integer(1)),
        Token::Operator(Operator::Plus),
        Token::Literal(Literal::Integer(2)),
        Token::Newline,

        Token::Identifier(Identifier::Simple("c".to_string())),
        Token::Assign,
        Token::Literal(Literal::Integer(3)),
        Token::Newline,

        Token::CloseBrace,
        Token::EndOfModule,
    ];

    assert_eq!(tokens, expected);
}