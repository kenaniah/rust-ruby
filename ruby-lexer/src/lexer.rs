#[cfg(test)]
mod tests;

mod core;
mod identifiers;
mod lex_state;
mod numbers;
mod whitespace;

use crate::plugins::NewlinesHandler;
use crate::*;

use lex_state::LexState;

use env_logger;
//use std::collections::HashMap;
use std::collections::VecDeque;

/// The number of characters held by the lexer's buffer
pub const BUFFER_SIZE: usize = 12;

/// Holds the lexer's current state
pub struct Lexer<T: Iterator<Item = char>> {
    input: T,
    nesting_level: usize,
    chr: VecDeque<Option<char>>,
    location: Location,
    //keywords: HashMap<String, Token>,
    prev_lex_state: LexState,
    lex_state: LexState,
    parsing_heredoc: bool,
    lex_strterm: bool,
    seen_whitespace: bool,
    /// Tracks whether the previous token was considered the start of a command
    prev_command_state: bool,
    /// Tracks whether the upcoming token may be considered the start of a command
    command_state: bool,
}

impl<T> Lexer<T>
where
    T: Iterator<Item = char>,
{
    /// Initializes a lexer and pre-reads the buffered number of characters
    pub fn new(input: T) -> Self {
        let _ = env_logger::builder().is_test(true).try_init();
        let mut lxr = Lexer {
            input: input,
            nesting_level: 0,
            chr: VecDeque::with_capacity(BUFFER_SIZE),
            location: Location::new(0, 0),
            //keywords: get_keywords(),
            prev_lex_state: LexState::EXPR_BEG,
            lex_state: LexState::EXPR_BEG,
            parsing_heredoc: false,
            lex_strterm: false,
            seen_whitespace: false,
            prev_command_state: false,
            command_state: false,
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
        self.prev_command_state = self.command_state;
        self.command_state = false;
        while let Some(c) = self.char(0) {
            self.prev_lex_state = self.lex_state;
            // TODO: check if we're in a string first
            // TODO: parse.y:4573
            // TODO: parse.y:4586

            // Handle whitespace
            if Self::is_whitespace(c) {
                return self.lex_whitespace();
            }

            match c {
                '#' => {
                    // found a comment
                    return self.lex_single_line_comment();
                }
                '\n' => {
                    // TODO: parse.y:4606
                    match self.lex_state {
                        LexState::EXPR_BEG | LexState::EXPR_FNAME | LexState::EXPR_DOT => {
                            if !self.parsing_heredoc && self.lex_strterm {
                                // self.parse_string();
                                todo!();
                                break;
                            }
                            // newline is not significant here
                            return self.emit_from_chars(Token::Newline, 1);
                        }
                        LexState::EXPR_CLASS | LexState::EXPR_VALUE => {
                            if !self.parsing_heredoc && self.lex_strterm {
                                // self.parse_string();
                                todo!();
                                break;
                            }
                        }
                        _ => {}
                    }
                    if !self.parsing_heredoc {
                        // newline is significant
                        return self.emit_from_chars(Token::LineTerminator, 1);
                    }
                    // TODO: parse.y:4754
                    todo!()
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
                            return self.emit_from_chars(Token::TwoStar, 2);
                        } else if self.is_beg() {
                            return self.emit_from_chars(Token::TwoStar, 2);
                        } else {
                            return self.emit_from_chars(Token::OpExponent, 2);
                        }
                    }
                    // *=
                    else if self.char(1) == Some('=') {
                        return self.emit_from_chars(
                            Token::AssignmentOperator {
                                value: "*=".to_owned(),
                            },
                            2,
                        );
                    }
                    // *
                    else if self.is_spcarg(c) {
                        self.warn("'*' interpreted as argument prefix");
                        return self.emit_from_chars(Token::Star, 1);
                    } else if self.is_beg() {
                        return self.emit_from_chars(Token::Star, 1);
                    } else {
                        return self.emit_from_chars(Token::OpMultiply, 1);
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
                    todo!()
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
                    todo!()
                }
                '\'' => {
                    // parse.y:4825
                    todo!()
                }
                '`' => {
                    // parse.y:4829
                    todo!()
                }
                '?' => {
                    // parse.y:4844
                    todo!()
                }
                '&' => {
                    // parse.y:4912
                    todo!()
                }
                '|' => {
                    // parse.y:4951
                    todo!()
                }
                '+' => {
                    // TODO: parse.y:4976
                    if Self::is_digit(self.char(1), 10) {
                        return self.lex_number();
                    }
                    todo!()
                }
                '-' => {
                    // parse.y:5004
                    if Self::is_digit(self.char(1), 10) {
                        return self.lex_number();
                    }
                    todo!()
                }
                '.' => {
                    // parse.y:5035
                    self.lex_state = LexState::EXPR_BEG;
                    if self.char(1) == Some('.') {
                        if self.char(2) == Some('.') {
                            return self.emit_from_chars(Token::ThreeDot, 3);
                        }
                        return self.emit_from_chars(Token::TwoDot, 2);
                    }
                    if Self::is_digit(self.char(1), 10) {
                        return Err(LexicalError {
                            message: "no .<digit> floating literal anymore; put 0 before dot"
                                .to_owned(),
                            location: self.get_pos(),
                        });
                    }

                    self.lex_state = LexState::EXPR_DOT;
                    return self.emit_from_chars(Token::Dot, 1);
                }
                '0'..='9' => {
                    // parse.y:5052
                    return self.lex_number();
                }
                ')' | ']' | '}' => {
                    // TODO: parse.y:5284
                    self.nesting_level -= 1;
                    self.lex_state = if c == ')' {
                        LexState::EXPR_ENDFN
                    } else {
                        LexState::EXPR_END
                    };
                    let token = match c {
                        ')' => Token::RightParen,
                        ']' => Token::RightBracket,
                        '}' => Token::RightBrace,
                        _ => todo!(),
                    };
                    return self.emit_from_chars(token, 1);
                }
                ':' => {
                    // TODO: parse.y:5297
                    todo!()
                }
                '/' => {
                    // TODO: parse.y:5321
                    todo!()
                }
                '^' => {
                    // TODO: parse.y:5344
                    todo!()
                }
                ';' => {
                    // parse.y:5359
                    self.lex_state = LexState::EXPR_BEG;
                    return self.emit_from_chars(Token::Semicolon, 1);
                }
                ',' => {
                    // parse.y:5359
                    self.lex_state = LexState::EXPR_BEG;
                    return self.emit_from_chars(Token::Comma, 1);
                }
                '~' => {
                    // parse.y:5367
                    self.set_lexer_newline_state();
                    return self.emit_from_chars(Token::OpBinComplement, 1);
                }
                '(' => {
                    // TODO: parse.y:5379
                    todo!()
                }
                '[' => {
                    // TODO: parse.y:5395
                    todo!()
                }
                '{' => {
                    // TODO: parse.y:5420
                    todo!()
                }
                '\\' => {
                    // parse.y:5440
                    if self.char(1) == Some('\n') {
                        return self.lex_whitespace();
                    }
                    return self.emit_from_chars(Token::Backslash, 1);
                }
                '%' => {
                    // TODO: parse.y:5451
                    todo!()
                }
                '$' => {
                    // TODO: parse.y:5539
                    todo!()
                }
                '@' => {
                    // parse.y:5633
                    let mut idx = 1;
                    if self.char(1) == Some('@') {
                        idx = 2;
                    }
                    match self.char(idx) {
                        None => {
                            let message = if idx == 1 {
                                "incomplete instance variable syntax"
                            } else {
                                "incomplete class variable syntax"
                            };
                            return Err(LexicalError {
                                message: message.to_owned(),
                                location: self.get_pos(),
                            });
                        }
                        Some(c @ '0'..='9') => {
                            let message = if idx == 1 {
                                format!("'@{}' is not allowed as an instance variable name", c)
                            } else {
                                format!("'@@{}' is not allowed as a class variable name", c)
                            };
                            return Err(LexicalError {
                                message: message.to_owned(),
                                location: self.get_pos(),
                            });
                        }
                        Some(c) => {
                            if Self::is_identchar(c) {
                                let prefix = if idx == 1 { "@" } else { "@@" };
                                return self.lex_identifier(prefix.to_owned());
                            }
                            return self.emit_from_chars(Token::At, 1);
                        }
                    }
                }
                _ => {
                    // TODO: parse.y:5679
                    return self.lex_identifier("".to_owned());
                }
            }
        }
        // End of file
        Ok((self.get_pos(), Token::EndOfFile, self.get_pos()))
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
                    if Self::is_whitespace(char) || char == '\n' {
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
                        message: "Multi-line comment was not terminated before the end of the file"
                            .to_owned(),
                        location: self.get_pos(),
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
// pub fn get_keywords() -> HashMap<String, Token> {
//     let mut keywords: HashMap<String, Token> = HashMap::new();
//     keywords.insert(String::from("__LINE__"), Token::KwLINE);
//     keywords.insert(String::from("__ENCODING__"), Token::KwENCODING);
//     keywords.insert(String::from("__FILE__"), Token::KwFILE);
//     keywords.insert(String::from("BEGIN"), Token::KwBEGIN);
//     keywords.insert(String::from("END"), Token::KwEND);
//     keywords.insert(String::from("alias"), Token::KwAlias);
//     keywords.insert(String::from("and"), Token::KwAnd);
//     keywords.insert(String::from("begin"), Token::KwBegin);
//     keywords.insert(String::from("break"), Token::KwBreak);
//     keywords.insert(String::from("case"), Token::KwCase);
//     keywords.insert(String::from("class"), Token::KwClass);
//     keywords.insert(String::from("def"), Token::KwDef);
//     keywords.insert(String::from("defined?"), Token::KwDefined);
//     keywords.insert(String::from("do"), Token::KwDo);
//     keywords.insert(String::from("else"), Token::KwElse);
//     keywords.insert(String::from("elsif"), Token::KwElsif);
//     keywords.insert(String::from("end"), Token::KwEnd);
//     keywords.insert(String::from("ensure"), Token::KwEnsure);
//     keywords.insert(String::from("for"), Token::KwFor);
//     keywords.insert(String::from("false"), Token::KwFalse);
//     keywords.insert(String::from("if"), Token::KwIf);
//     keywords.insert(String::from("in"), Token::KwIn);
//     keywords.insert(String::from("module"), Token::KwModule);
//     keywords.insert(String::from("next"), Token::KwNext);
//     keywords.insert(String::from("nil"), Token::KwNil);
//     keywords.insert(String::from("not"), Token::KwNot);
//     keywords.insert(String::from("or"), Token::KwOr);
//     keywords.insert(String::from("redo"), Token::KwRedo);
//     keywords.insert(String::from("rescue"), Token::KwRescue);
//     keywords.insert(String::from("retry"), Token::KwRetry);
//     keywords.insert(String::from("return"), Token::KwReturn);
//     keywords.insert(String::from("self"), Token::KwSelf);
//     keywords.insert(String::from("super"), Token::KwSuper);
//     keywords.insert(String::from("then"), Token::KwThen);
//     keywords.insert(String::from("true"), Token::KwTrue);
//     keywords.insert(String::from("undef"), Token::KwUndef);
//     keywords.insert(String::from("unless"), Token::KwUnless);
//     keywords.insert(String::from("until"), Token::KwUntil);
//     keywords.insert(String::from("when"), Token::KwWhen);
//     keywords.insert(String::from("while"), Token::KwWhile);
//     keywords.insert(String::from("yield"), Token::KwYield);
//     keywords
// }
