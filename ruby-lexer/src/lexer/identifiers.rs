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
}
