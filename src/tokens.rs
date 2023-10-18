use std::{ path::Display};

use crate::elements::{Identifier, Literal, Operator, Keyword};

#[derive(Debug,  Clone, PartialEq)]
pub enum Token {
    // Symbols
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,

    ListSeparator,
    Dot,
    Colon,

    Newline,
    EndOfModule,

    // Operators
    Operator(Operator),
    TernaryCondition,
    Assign,

    Keyword(Keyword),
    Literal(Literal),
    Identifier(Identifier),
}
