#[macro_use]
extern crate bitflags;

mod error;
mod lexer;
mod location;
pub mod plugins;
mod tokens;

pub use error::{LexicalError, LexicalErrorType};
pub use location::Location;
pub use tokens::Token;
pub use lexer::Lexer;

/// Composite type that tracks a token and its starting and ending location
pub type Spanned = (Location, Token, Location);

/// Type used to track the success of a lexing operation
pub type LexResult = Result<Spanned, LexicalError>;
