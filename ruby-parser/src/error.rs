use super::location::Location;
use super::tokens::Token;
use lalrpop_util::ParseError as LalrpopError;

use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct LexicalError {
    pub error: LexicalErrorType,
    pub location: Location,
}

#[derive(Debug, PartialEq)]
pub enum LexicalErrorType {
    StringError,
    UnicodeError,
    NestingError,
    UnrecognizedToken { token: char },
}

impl From<LexicalError> for LalrpopError<Location, Token, LexicalError> {
    fn from(err: LexicalError) -> Self {
        lalrpop_util::ParseError::User { error: err }
    }
}
