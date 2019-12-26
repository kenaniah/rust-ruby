use super::{LexResult, Lexer, Token};

impl<T> Lexer<T>
where
    T: Iterator<Item = char>,
{
    /// Helper function to determine if a character is whitespace (not including newline)
    pub fn is_whitespace(c: char) -> bool {
        match c {
            ' ' | '\t' | '\x0b' | '\x0c' | '\r' => true,
            _ => false,
        }
    }

    /// Lexes a sequence of whitespace characters and escaped newlines
    pub fn lex_whitespace(&mut self) -> LexResult {
        let tok_start = self.get_pos();
        loop {
            if let Some(c) = self.char(0) {
                if Self::is_whitespace(c) {
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
        Ok((tok_start, Token::Whitespace, self.get_pos()))
    }
}
