use std::{ path::Display};

use crate::elements::{Identifier, Literal, Operator, Keyword};

#[derive(Debug,  Clone, PartialEq)]
pub enum Token {
    // Symbols
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenSquareBracket,
    CloseSquareBracket,

    ListSeparator,
    Dot,
    Colon,

    Newline,
    EndOfModule,

    // Operators
    Operator(Operator),
    TernaryCondition,
    Assign,
    BinaryAssign(Operator),

    Keyword(Keyword),
    Literal(Literal),
    Identifier(Identifier),
}
