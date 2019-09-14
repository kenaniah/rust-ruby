//! This module handles the lexing of Ruby source code. The input will be translated into a
//! stream of lexed [`Token`](../tokens/enum.Token.html)s for use in the parser.

use super::error::{LexicalError, LexicalErrorType};
use super::location::Location;
pub use super::tokens::Token;
use log::trace;
use std::collections::HashMap;
use unicode_xid::UnicodeXID;

/// Composite type that tracks a token and its starting and ending location
pub type Spanned = (Location, Token, Location);

/// Type used to track the success of a lexing operation
pub type LexResult = Result<Spanned, LexicalError>;

/// Holds the lexer's current state
pub struct Lexer<T: Iterator<Item = char>> {
    chars: T,
    at_line_start: bool,
    nesting_level: usize,
    pending_tokens: Vec<Spanned>,
    chr0: Option<char>,
    chr1: Option<char>,
    chr2: Option<char>,
    location: Location,
    keywords: HashMap<String, Token>,
}

pub fn make_tokenizer(source: &str) -> impl Iterator<Item = LexResult> + '_ {
    Lexer::new(source.chars())
}

// 8.7.2 - Keywords (alphanumerically)
pub fn get_keywords() -> HashMap<String, Token> {
    let mut keywords: HashMap<String, Token> = HashMap::new();
    keywords.insert(String::from("__LINE__"), Token::KwLINE);
    keywords.insert(String::from("__ENCODING__"), Token::KwENCODING);
    keywords.insert(String::from("__FILE__"), Token::KwFILE);
    keywords.insert(String::from("BEGIN"), Token::KwBEGIN);
    keywords.insert(String::from("END"), Token::KwEND);
    keywords.insert(String::from("alias"), Token::KwAlias);
    keywords.insert(String::from("and"), Token::KwAnd);
    keywords.insert(String::from("begin"), Token::KwBegin);
    keywords.insert(String::from("break"), Token::KwBreak);
    keywords.insert(String::from("case"), Token::KwCase);
    keywords.insert(String::from("class"), Token::KwClass);
    keywords.insert(String::from("def"), Token::KwDef);
    keywords.insert(String::from("defined?"), Token::KwDefined);
    keywords.insert(String::from("do"), Token::KwDo);
    keywords.insert(String::from("else"), Token::KwElse);
    keywords.insert(String::from("elsif"), Token::KwElsif);
    keywords.insert(String::from("end"), Token::KwEnd);
    keywords.insert(String::from("ensure"), Token::KwEnsure);
    keywords.insert(String::from("for"), Token::KwFor);
    keywords.insert(String::from("false"), Token::KwFalse);
    keywords.insert(String::from("if"), Token::KwIf);
    keywords.insert(String::from("in"), Token::KwIn);
    keywords.insert(String::from("module"), Token::KwModule);
    keywords.insert(String::from("next"), Token::KwNext);
    keywords.insert(String::from("nil"), Token::KwNil);
    keywords.insert(String::from("not"), Token::KwNot);
    keywords.insert(String::from("or"), Token::KwOr);
    keywords.insert(String::from("redo"), Token::KwRedo);
    keywords.insert(String::from("rescue"), Token::KwRescue);
    keywords.insert(String::from("retry"), Token::KwRetry);
    keywords.insert(String::from("return"), Token::KwReturn);
    keywords.insert(String::from("self"), Token::KwSelf);
    keywords.insert(String::from("super"), Token::KwSuper);
    keywords.insert(String::from("then"), Token::KwThen);
    keywords.insert(String::from("true"), Token::KwTrue);
    keywords.insert(String::from("undef"), Token::KwUndef);
    keywords.insert(String::from("unless"), Token::KwUnless);
    keywords.insert(String::from("until"), Token::KwUntil);
    keywords.insert(String::from("when"), Token::KwWhen);
    keywords.insert(String::from("while"), Token::KwWhile);
    keywords.insert(String::from("yield"), Token::KwYield);
    keywords
}

impl<T> Lexer<T>
where
    T: Iterator<Item = char>,
{
    /// Initializes a lexer and pre-reads the first 3 characters
    pub fn new(input: T) -> Self {
        let mut lxr = Lexer {
            chars: input,
            at_line_start: true,
            nesting_level: 0,
            pending_tokens: Vec::new(),
            chr0: None,
            chr1: None,
            chr2: None,
            location: Location::new(0, 0),
            keywords: get_keywords(),
        };
        // Preload the first 3 characters into the lexer
        lxr.next_char();
        lxr.next_char();
        lxr.next_char();
        lxr.location.reset(); // Moves back to line 1, col 1
        lxr
    }

    /// This function is used by the iterator implementation to retrieve the next token
    fn inner_next(&mut self) -> LexResult {
        while self.pending_tokens.is_empty() {
            self.consume_normal()?;
        }
        Ok(self.pending_tokens.remove(0))
    }

    /// This function takes a look at the next character, if any, and decides on the next steps
    fn consume_normal(&mut self) -> Result<(), LexicalError> {
        // Check if we have some character
        if let Some(c) = self.chr0 {
            // First check for an identifier
            if self.is_identifier_start(c) {
                let identifier = self.lex_identifier()?;
                self.emit(identifier);
            } else {
                self.consume_character(c)?;
            }
        } else {
            // We're at the end of the file
            self.emit((self.get_pos(), Token::EndOfFile, self.get_pos()));
        }
        Ok(())
    }

    /// Consumes non-identifying characters
    fn consume_character(&mut self, c: char) -> Result<(), LexicalError> {
        let tok_start = self.get_pos();
        match c {
            '\x09' | '\x0b' | '\x0c' | '\x0d' | '\x20' => {
                // Consume whitespaces
                self.next_char();
                while self.chr0 == Some('\x09') || self.chr0 == Some('\x0b') || self.chr0 == Some('\x0c') || self.chr0 == Some('\x0d') || self.chr0 == Some('\x20') {
                    self.next_char();
                }
                let tok_end = self.get_pos();
                self.emit((tok_start, Token::Whitespace, tok_end))
            }
            _ => {
                let c = self.next_char();
                return Err(LexicalError {
                    location: tok_start,
                    error: LexicalErrorType::UnrecognizedToken { token: c.unwrap() }
                })
            }
        }
        Ok(())
    }

    /// Helper function that consumes the next upcoming character
    /// This method will also adjust the lexer's current location accordingly
    fn next_char(&mut self) -> Option<char> {
        // Shift the stack of upcoming characters
        let c = self.chr0;
        self.chr0 = self.chr1;
        self.chr1 = self.chr2;
        self.chr2 = self.chars.next();

        // Update the lexer's source location
        if c == Some('\n') {
            self.location.newline();
        } else {
            self.location.move_right();
        }
        c
    }

    /// Helper function to retrieve the lexer's current location
    fn get_pos(&self) -> Location {
        self.location.clone()
    }

    /// Helper function to emit a lexed token to the queue of tokens
    fn emit(&mut self, spanned: Spanned) {
        self.pending_tokens.push(spanned);
    }

    /// Determines whether this character is a valid starting unicode identifier
    fn is_identifier_start(&self, c: char) -> bool {
        match c {
            '_' => true,
            c => UnicodeXID::is_xid_start(c),
        }
    }

    /// Determines whether the character is the continuation of a valid unicode identifier
    fn is_identifier_continuation(&self) -> bool {
        if let Some(c) = self.chr0 {
            match c {
                '_' | '0'..='9' => true,
                c => UnicodeXID::is_xid_continue(c),
            }
        } else {
            false
        }
    }

    fn lex_identifier(&mut self) -> LexResult {
        let mut name = String::new();
        let start_pos = self.get_pos();

        // Take the first character
        name.push(self.next_char().unwrap());

        // Check for more identifier characters
        while self.is_identifier_continuation() {
            name.push(self.next_char().unwrap());
        }
        let end_pos = self.get_pos();

        // Emit the token
        if self.keywords.contains_key(&name) {
            Ok((start_pos, self.keywords[&name].clone(), end_pos))
        } else {
            Ok((
                start_pos,
                Token::RefactorIdentifier { value: name },
                end_pos,
            ))
        }
    }
}

impl<T> Iterator for Lexer<T>
where
    T: Iterator<Item = char>,
{
    type Item = LexResult;
    fn next(&mut self) -> Option<Self::Item> {
        let token = self.inner_next();
        trace!("Lex token {:?}, nesting={:?}", token, self.nesting_level);
        match token {
            Ok((_, Token::EndOfFile, _)) => None,
            r => Some(r),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{make_tokenizer, Token};
    use std::iter::FromIterator;
    use std::iter::Iterator;

    pub fn lex_source(source: &String) -> Vec<Token> {
        let lexer = make_tokenizer(source);
        Vec::from_iter(lexer.map(|x| x.unwrap().1))
    }

    #[test]
    fn test_basics() {
        let source = String::from("foo bar");
        let tokens = lex_source(&source);
        assert_eq!(tokens, vec![
            Token::RefactorIdentifier { value: String::from("foo") },
            Token::Whitespace,
            Token::RefactorIdentifier { value: String::from("bar") }
        ])
    }

}
