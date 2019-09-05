//! This module handles the lexing of Ruby source code. The input will be translated into a
//! stream of lexed [`Token`](../tokens/enum.Token.html)s for use in the parser.

// stdlib
use std::collections::HashMap;
use core::fmt::Error; // temporary

pub use super::tokens::Token;
use super::location::Location;

/// Composite type that tracks a token and its starting and ending location
pub type Spanned = (Location, Token, Location);

/// Type used to track the success of a lexing operation
pub type LexResult = Result<Spanned, Error>;

/// Holds the lexer's current state
pub struct Lexer<T: Iterator<Item = char>> {
    chars: T,
    at_line_start: bool,
    nesting_level: usize,
    pending_tokens: Vec<Spanned>,
    char0: Option<char>,
    char1: Option<char>,
    char2: Option<char>,
    location: Location,
    keywords: HashMap<String, Token>
}

// 8.7.2 - Keywords (alphanumerically)
pub fn get_keywords() -> HashMap<String, Token> {
    let mut keywords: HashMap<String, Token> = HashMap::new();
    keywords.insert(String::from("__LINE__"), Token::LINE);
    keywords.insert(String::from("__ENCODING__"), Token::ENCODING);
    keywords.insert(String::from("__FILE__"), Token::FILE);
    keywords.insert(String::from("BEGIN"), Token::BEGIN);
    keywords.insert(String::from("END"), Token::END);
    keywords.insert(String::from("alias"), Token::Alias);
    keywords.insert(String::from("and"), Token::And);
    keywords.insert(String::from("begin"), Token::Begin);
    keywords.insert(String::from("break"), Token::Break);
    keywords.insert(String::from("case"), Token::Case);
    keywords.insert(String::from("class"), Token::Class);
    keywords.insert(String::from("def"), Token::Def);
    keywords.insert(String::from("defined?"), Token::Defined);
    keywords.insert(String::from("do"), Token::Do);
    keywords.insert(String::from("else"), Token::Else);
    keywords.insert(String::from("elsif"), Token::Elsif);
    keywords.insert(String::from("end"), Token::End);
    keywords.insert(String::from("ensure"), Token::Ensure);
    keywords.insert(String::from("for"), Token::For);
    keywords.insert(String::from("false"), Token::False);
    keywords.insert(String::from("if"), Token::If);
    keywords.insert(String::from("in"), Token::In);
    keywords.insert(String::from("module"), Token::Module);
    keywords.insert(String::from("next"), Token::Next);
    keywords.insert(String::from("nil"), Token::Nil);
    keywords.insert(String::from("not"), Token::Not);
    keywords.insert(String::from("or"), Token::Or);
    keywords.insert(String::from("redo"), Token::Redo);
    keywords.insert(String::from("rescue"), Token::Rescue);
    keywords.insert(String::from("retry"), Token::Retry);
    keywords.insert(String::from("return"), Token::Return);
    keywords.insert(String::from("self"), Token::Slf);
    keywords.insert(String::from("super"), Token::Super);
    keywords.insert(String::from("then"), Token::Then);
    keywords.insert(String::from("true"), Token::True);
    keywords.insert(String::from("undef"), Token::Undef);
    keywords.insert(String::from("unless"), Token::Unless);
    keywords.insert(String::from("until"), Token::Until);
    keywords.insert(String::from("when"), Token::When);
    keywords.insert(String::from("while"), Token::While);
    keywords.insert(String::from("yield"), Token::Yield);
    keywords
}
