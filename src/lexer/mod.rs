mod tokenizer;
mod preprocessor;

use crate::tokens::Token;


pub fn tokenize(src: &str) -> Vec<Token> {
    preprocessor::preprocess(&tokenizer::tokenize(src))
}
