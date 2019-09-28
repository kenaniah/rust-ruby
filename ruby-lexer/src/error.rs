use crate::Location;
use crate::Token;

#[derive(Debug, PartialEq)]
pub struct LexicalError {
    pub message: String,
    pub error: LexicalErrorType,
    pub location: Location,
}

#[derive(Debug, PartialEq)]
pub enum LexicalErrorType {
    LexingError,
    StringError,
    UnicodeError,
    NestingError,
    UnrecognizedToken { token: char },
    UnterminatedMultilineComment,
}

// use lalrpop_util::ParseError as LalrpopError;
// impl From<LexicalError> for LalrpopError<Location, Token, LexicalError> {
//     fn from(err: LexicalError) -> Self {
//         lalrpop_util::ParseError::User { error: err }
//     }
// }
