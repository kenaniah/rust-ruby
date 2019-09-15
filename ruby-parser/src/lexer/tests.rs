use super::{make_tokenizer, Token};
use std::iter::FromIterator;
use std::iter::Iterator;

// TODO: change this to a Result<Vec<Token>, LexicalError>
pub fn lex_source(source: &String) -> Vec<Token> {
    let lexer = make_tokenizer(source);
    Vec::from_iter(lexer.map(|x| x.unwrap().1))
}

mod comment;
mod whitespace;
