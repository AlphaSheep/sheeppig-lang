use std::iter::Peekable;
use std::slice::Iter;

use crate::elements::Identifier;
use crate::tokens::Token;
use crate::tree;

use crate::parser::utils::handle_parse_error;


pub fn parse_using_block(tokens: &mut Peekable<Iter<Token>>) -> tree::Import {
    panic!("Not implemented");
}
