use super::{BUFFER_SIZE, LexResult, LexState, Lexer, Location, SpannedToken, Token};
use log::trace;

impl<T> Lexer<T>
where
    T: Iterator<Item = char>,
{
    /// Checks if the given character is a valid Ruby identifier character
    ///
    /// Identifying characters include `[a-zA-Z0-9_]` and non-ascii characters
    pub fn is_identchar(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_' || !c.is_ascii()
    }

    /// Lexes and returns an identifier or language keyword
    pub fn lex_identifier(&mut self) -> LexResult {

        unimplemented!()
    }

    // Lexes a named identifier
    // fn lex_identifier(&mut self) -> LexResult {
    //     let mut name = String::new();
    //     let start_pos = self.get_pos();
    //
    //     // Take the first character
    //     name.push(self.next_char().unwrap());
    //
    //     // Check for more identifier characters
    //     while self.is_identifier_continuation() {
    //         name.push(self.next_char().unwrap());
    //     }
    //
    //     // Check for an ending ? or ! (valid for method names)
    //     if self.char(0) == Some('?') || self.char(0) == Some('!') {
    //         name.push(self.next_char().unwrap());
    //     }
    //
    //     let end_pos = self.get_pos();
    //
    //     // Emit the token
    //     if self.keywords.contains_key(&name) {
    //         Ok((start_pos, self.keywords[&name].clone(), end_pos))
    //     } else {
    //         Ok((
    //             start_pos,
    //             Token::RefactorIdentifier { value: name },
    //             end_pos,
    //         ))
    //     }
    // }

}
