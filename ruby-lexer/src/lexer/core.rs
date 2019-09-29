use super::{BUFFER_SIZE, LexResult, LexState, Lexer, Location, SpannedToken, Token};
use log::trace;

impl<T> Lexer<T>
where
    T: Iterator<Item = char>,
{
    /// This function is used by the iterator implementation to retrieve the next token.
    ///
    /// Depending on what type of token is returned, the lexing state may be adjusted.
    pub fn inner_next(&mut self) -> LexResult {
        let lex_result = self.produce_token();
        if let Ok((_, token, _)) = &lex_result {
            match token {
                // Assignments should always mark the start of an expression
                Token::AssignmentOperator { value: _ } => {
                    self.lex_state = LexState::EXPR_BEG;
                    self.seen_whitespace = false;
                }
                Token::Whitespace => {
                    self.seen_whitespace = true;
                }
                _ => {
                    self.seen_whitespace = false;
                }
            }
        }
        lex_result
    }

    /// Returns the character at the given index within the lexer's buffer.
    pub fn char(&self, index: usize) -> Option<char> {
        assert!(index < BUFFER_SIZE);
        match self.chr.get(index) {
            Some(c) => *c,
            None => None,
        }
    }

    /// Returns the next `n` number of characters as a string (or `None` if EOF encountered).
    pub fn chars(&self, n: usize) -> Option<String> {
        let mut str = String::with_capacity(n);
        for i in 0..n {
            match self.chr.get(i) {
                Some(Some(c)) => str.push(*c),
                _ => return None,
            }
        }
        Some(str)
    }

    /// Consumes and returns the next upcoming character, adjusting the lexer's current location.
    pub fn next_char(&mut self) -> Option<char> {
        // Shift the stack of upcoming characters
        let c = self.chr.pop_front()?;
        self.chr.push_back(self.input.next());

        // Update the lexer's source location
        if c == Some('\n') {
            self.location.newline();
        } else if let Some(_) = c {
            self.location.move_right();
        }
        c
    }

    /// Retrieve's the lexer's current location in the source stream.
    pub fn get_pos(&self) -> Location {
        self.location.clone()
    }

    /// Emits tokens from one or more characters in the buffer.
    ///
    /// # Panics
    ///  * Panics if the number of characters requested is greater than the buffer's size.
    ///  * Panics if the number of characters requested moves past the end of the buffer's input stream.
    pub fn emit_from_chars(&mut self, token: Token, chars: usize) -> LexResult {
        let tok_start = self.get_pos();
        match chars {
            1..=BUFFER_SIZE => {
                for _ in 1..=chars {
                    self.next_char().unwrap();
                }
            }
            _ => panic!("emit_from_chars can only consume up to {} characters at a time", BUFFER_SIZE),
        }
        Ok((tok_start, token, self.get_pos()))
    }
}

impl<T> Iterator for Lexer<T>
where
    T: Iterator<Item = char>,
{
    type Item = LexResult;
    /// Produces a `LexResult` unless the end of the file was reached
    fn next(&mut self) -> Option<Self::Item> {
        let token = self.inner_next();
        trace!("Lex token {:?}, nesting={:?}", token, self.nesting_level);
        match token {
            Ok((_, Token::EndOfFile, _)) => None,
            r => Some(r),
        }
    }
}
