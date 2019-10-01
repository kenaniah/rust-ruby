use crate::Location;

#[derive(Debug, PartialEq)]
pub struct LexicalError {
    pub message: String,
    pub location: Location,
}

// use lalrpop_util::ParseError as LalrpopError;
// impl From<LexicalError> for LalrpopError<Location, Token, LexicalError> {
//     fn from(err: LexicalError) -> Self {
//         lalrpop_util::ParseError::User { error: err }
//     }
// }
