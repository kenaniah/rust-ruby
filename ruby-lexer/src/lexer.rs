//#[cfg(test)]
//mod tests;

mod core;
mod lex_state;

use crate::plugins::NewlinesHandler;
use crate::*;

use lex_state::LexState;

use std::collections::HashMap;
use std::collections::VecDeque;
use unicode_xid::UnicodeXID;

/// The number of characters held by the lexer's buffer
pub const BUFFER_SIZE: usize = 12;

/// Holds the lexer's current state
pub struct Lexer<T: Iterator<Item = char>> {
    input: T,
    nesting_level: usize,
    pending_tokens: Vec<SpannedToken>,
    chr: VecDeque<Option<char>>,
    location: Location,
    keywords: HashMap<String, Token>,
    lex_state: LexState,
    parsing_heredoc: bool,
    lex_strterm: bool,
    seen_whitespace: bool,
}

impl<T> Lexer<T>
where
    T: Iterator<Item = char>,
{
    /// Initializes a lexer and pre-reads the buffered number of characters
    pub fn new(input: T) -> Self {
        let mut lxr = Lexer {
            input: input,
            nesting_level: 0,
            pending_tokens: Vec::new(),
            chr: VecDeque::with_capacity(BUFFER_SIZE),
            location: Location::new(0, 0),
            keywords: get_keywords(),
            lex_state: LexState::EXPR_BEG,
            parsing_heredoc: false,
            lex_strterm: false,
            seen_whitespace: false,
        };
        // Preload the lexer's buffer
        for _ in 1..=BUFFER_SIZE {
            lxr.chr.push_back(lxr.input.next());
        }
        lxr.location.reset(); // Moves to line 1, col 1
        lxr
    }

    /// This function takes a look at the next character, if any, and emits the relevant token
    fn produce_token(&mut self) -> LexResult {
        self.seen_whitespace = false;

        while let Some(c) = self.char(0) {
            // TODO: check if we're in a string first
            // TODO: parse.y:4573
            // TODO: parse.y:4586

            // Handle whitespace
            if self.is_whitespace(c) {
                self.seen_whitespace = true;
                self.next_char();
                continue;
            }

            match c {
                '#' => {
                    // found a comment
                    return self.lex_single_line_comment();
                }
                '\n' => {
                    // TODO: parse.y:4606
                    match self.lex_state {
                        LexState::EXPR_BEG
                        | LexState::EXPR_FNAME
                        | LexState::EXPR_DOT
                        | LexState::EXPR_CLASS
                        | LexState::EXPR_VALUE => {
                            if !self.parsing_heredoc && self.lex_strterm {
                                // self.parse_string();
                                unimplemented!();
                                break;
                            }
                            self.next_char();
                            continue;
                        }
                        _ => {}
                    }
                    if !self.parsing_heredoc {
                        return self.emit_from_chars(Token::LineTerminator, 1);
                    }
                    // TODO: parse.y:4754
                    unimplemented!()
                }
                '*' => {
                    // parse.y:4652
                    self.set_lexer_newline_state();
                    // **=
                    if self.chars(3) == Some("**=".to_owned()) {
                        return self.emit_from_chars(
                            Token::AssignmentOperator {
                                value: "**=".to_owned(),
                            },
                            3,
                        );
                    }
                    // **
                    else if self.char(1) == Some('*') {
                        if self.is_spcarg(c) {
                            self.warn("'**' interpreted as argument prefix");
                            return self.emit_from_chars(Token::TwoStar, 2)
                        } else if self.is_beg() {
                            return self.emit_from_chars(Token::TwoStar, 2)
                        } else {
                            return self.emit_from_chars(Token::OpExponent, 2)
                        }
                    }
                    // *=
                    else if self.char(1) == Some('=') {
                        return self.emit_from_chars(
                            Token::AssignmentOperator {
                                value: "*=".to_owned(),
                            },
                            2,
                        )
                    }
                    // *
                    else if self.is_spcarg(c) {
                        self.warn("'*' interpreted as argument prefix");
                        return self.emit_from_chars(Token::Star, 1)
                    } else if self.is_beg() {
                        return self.emit_from_chars(Token::Star, 1)
                    } else {
                        return self.emit_from_chars(Token::OpMultiply, 1)
                    };
                }
                '!' => {
                    // parse.y:4697
                    self.set_lexer_newline_state();
                    // !=
                    if self.char(1) == Some('=') {
                        return self.emit_from_chars(Token::OpNotEqual, 2);
                    }
                    // !~
                    if self.char(1) == Some('~') {
                        return self.emit_from_chars(Token::OpNotMatch, 2);
                    }
                    // !
                    return self.emit_from_chars(Token::OpNot, 1);
                }
                '=' => {
                    //  parse.y:4717
                    // TODO: parse.y:4718 (multi-line comments)
                    self.set_lexer_newline_state();
                    // ===
                    if self.chars(3) == Some("===".to_owned()) {
                        return self.emit_from_chars(Token::OpTripleEqual, 3);
                    }
                    // ==
                    else if self.char(1) == Some('=') {
                        return self.emit_from_chars(Token::OpDoubleEqual, 2);
                    }
                    // =~
                    else if self.char(1) == Some('~') {
                        return self.emit_from_chars(Token::OpMatch, 2);
                    }
                    // =>
                    else if self.char(1) == Some('>') {
                        return self.emit_from_chars(Token::Arrow, 2);
                    }
                    // =
                    return self.emit_from_chars(Token::OpAssign, 1);
                }
                '<' => {
                    // parse.y:4760
                    unimplemented!()
                }
                '>' => {
                    // parse.y:4799
                    self.set_lexer_newline_state();
                    // >=
                    if self.char(1) == Some('=') {
                        return self.emit_from_chars(Token::OpGtEqual, 2);
                    }
                    if self.char(1) == Some('>') {
                        // >>=
                        if self.char(2) == Some('=') {
                            return self.emit_from_chars(
                                Token::AssignmentOperator {
                                    value: ">>=".to_owned(),
                                },
                                3,
                            );
                        }
                        // >>
                        return self.emit_from_chars(Token::OpRightShift, 2);
                    }
                    return self.emit_from_chars(Token::OpGt, 1);
                }
                '"' => {
                    // parse.y:4821
                    unimplemented!()
                }
                '\'' => {
                    // parse.y:4825
                    unimplemented!()
                }
                '`' => {
                    // parse.y:4829
                    unimplemented!()
                }
                '?' => {
                    // parse.y:4844
                    unimplemented!()
                }
                '&' => {
                    // parse.y:4912
                    unimplemented!()
                }
                '|' => {
                    // parse.y:4951
                    unimplemented!()
                }
                _ => unimplemented!(),
            }
        }
        unimplemented!()
    }

    /// Consumes non-identifying characters
    // fn consume_non_identifier(&mut self, c: char) -> Result<(), LexicalError> {
    //     let tok_start = self.get_pos();
    //     match c {
    //         ' ' | '\t' | '\x0b' | '\x0c' | '\r' => {
    //             self.lex_whitespace();
    //         }
    //         '\n' => {
    //             self.emit_from_chars(Token::LineTerminator, 1);
    //         }
    //         '[' => {
    //             self.emit_from_chars(Token::LeftBracket, 1);
    //         }
    //         ']' => {
    //             self.emit_from_chars(Token::RightBracket, 1);
    //         }
    //         '\\' => {
    //             if self.char(1) == Some('\n') {
    //                 self.lex_whitespace();
    //             } else {
    //                 panic!("\\ is not handled yet");
    //             }
    //         }
    //         '#' => {
    //             self.lex_single_line_comment();
    //         }
    //         '=' => {
    //             // Check for the start of a multi-line comment
    //             if self.get_pos().col() == 1 && self.chars(6) == Some("=begin".to_owned()) {
    //                 if let Some(char) = self.char(6) {
    //                     if self.is_whitespace(char) || char == '\n' {
    //                         let comment = self.lex_multi_line_comment()?;
    //                         self.emit(comment);
    //                         return Ok(());
    //                     }
    //                 }
    //             }
    //             panic!("= is not handled yet")
    //         }
    //         _ => {
    //             let c = self.next_char();
    //             return Err(LexicalError {
    //                 location: tok_start,
    //                 error: LexicalErrorType::UnrecognizedToken { token: c.unwrap() },
    //             });
    //         }
    //     }
    //     Ok(())
    // }

    /// Determines whether this character is a valid starting unicode identifier
    fn is_identifier_start(&self, c: char) -> bool {
        match c {
            '_' => true,
            c => UnicodeXID::is_xid_start(c),
        }
    }

    /// Determines whether the character is the continuation of a valid unicode identifier
    fn is_identifier_continuation(&mut self) -> bool {
        if let Some(c) = self.char(0) {
            match c {
                '_' | '0'..='9' => true,
                c => UnicodeXID::is_xid_continue(c),
            }
        } else {
            false
        }
    }

    /// Lexes a named identifier
    fn lex_identifier(&mut self) -> LexResult {
        let mut name = String::new();
        let start_pos = self.get_pos();

        // Take the first character
        name.push(self.next_char().unwrap());

        // Check for more identifier characters
        while self.is_identifier_continuation() {
            name.push(self.next_char().unwrap());
        }

        // Check for an ending ? or ! (valid for method names)
        if self.char(0) == Some('?') || self.char(0) == Some('!') {
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

    /// Helper function to determine if a character is whitespace (not including newline)
    fn is_whitespace(&self, c: char) -> bool {
        match c {
            ' ' | '\t' | '\x0b' | '\x0c' | '\r' => true,
            _ => false,
        }
    }

    /// Lexes a sequence of whitespace characters and escaped newlines
    fn lex_whitespace(&mut self) -> LexResult {
        let tok_start = self.get_pos();
        if self.char(0) == Some('\n') {
            // Consume a line terminator
            self.emit_from_chars(Token::LineTerminator, 2)
        } else {
            // Consume whitespaces
            loop {
                if let Some(c) = self.char(0) {
                    if self.is_whitespace(c) {
                        // Handle a normal whitespace
                        self.next_char();
                        continue;
                    } else if c == '\\' && self.char(1) == Some('\n') {
                        // Handle line continuations
                        self.next_char();
                        self.next_char();
                        continue;
                    }
                }
                break;
            }
            let tok_end = self.get_pos();
            Ok((tok_start, Token::Whitespace, tok_end))
        }
    }

    /// Lexes a single-line comment
    fn lex_single_line_comment(&mut self) -> LexResult {
        let tok_start = self.get_pos();
        let mut content = String::new();
        self.next_char(); // Discard the '#'
        while self.char(0) != Some('\n') && self.char(0) != None {
            content.push(self.next_char().unwrap());
        }
        return Ok((tok_start, Token::Comment { value: content }, self.get_pos()));
    }

    /// Lexes a multi-line comment
    fn lex_multi_line_comment(&mut self) -> LexResult {
        let tok_start = self.get_pos();
        let mut str = String::new();

        // Discard the '=begin '
        for _ in 1..=7 {
            self.next_char();
        }

        // Grab everything until '=end ' is found at the beginning of a line
        loop {
            // Check for the end of the multi-line comment and break if found
            if self.get_pos().col() == 1 && self.chars(4) == Some("=end".to_owned()) {
                if let Some(char) = self.char(4) {
                    if self.is_whitespace(char) || char == '\n' {
                        // Discard the '=end '
                        for _ in 1..=5 {
                            self.next_char();
                        }
                        break;
                    }
                }
            }

            // Otherwise, consume the next character
            match self.next_char() {
                Some(c) => str.push(c),
                None => {
                    return Err(LexicalError {
                        location: self.get_pos(),
                        error: LexicalErrorType::UnterminatedMultilineComment,
                    })
                }
            }
        }

        // Consume the rest of the line
        loop {
            match self.char(0) {
                Some('\n') => break,
                Some(_) => str.push(self.next_char().unwrap()),
                None => break,
            }
        }

        // Return the lexed result
        Ok((
            tok_start,
            Token::Comment {
                value: str.to_owned(),
            },
            self.get_pos(),
        ))
    }

    fn warn(&self, _msg: &str) {
        // Do something with the string
    }
}

pub fn make_tokenizer(source: &str) -> impl Iterator<Item = LexResult> + '_ {
    let nlh = NewlinesHandler::new(source.chars());
    Lexer::new(nlh)
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
