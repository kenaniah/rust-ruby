use super::{BUFFER_SIZE, LexResult, LexState, Lexer, Location, SpannedToken, Token};
use log::trace;

impl<T> Lexer<T>
where
    T: Iterator<Item = char>,
{
    /// This function is used by the iterator implementation to retrieve the next token.
    pub fn inner_next(&mut self) -> LexResult {
        while self.pending_tokens.is_empty() {
            self.produce_token()?;
        }
        Ok(self.pending_tokens.remove(0))
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

    /// Emits a lexed token to the queue of tokens, potentially adjusting the lexing state.
    pub fn emit(&mut self, spanned: SpannedToken) {
        match spanned.1 {
            // Assignments should change the lexing state
            Token::AssignmentOperator { value: _ } => {
                self.lex_state = LexState::EXPR_BEG;
            }
            _ => {}
        }
        self.pending_tokens.push(spanned);
    }

    /// Emits tokens from one or more characters in the buffer.
    ///
    /// # Panics
    ///  * Panics if the number of characters requested is greater than the buffer's size.
    ///  * Panics if the number of characters requested moves past the end of the buffer's input stream.
    pub fn emit_from_chars(&mut self, token: Token, chars: usize) {
        let tok_start = self.get_pos();
        match chars {
            1..=BUFFER_SIZE => {
                for _ in 1..=chars {
                    self.next_char().unwrap();
                }
            }
            _ => panic!("emit_from_chars can only consume up to {} characters at a time", BUFFER_SIZE),
        }
        self.emit((tok_start, token, self.get_pos()));
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
