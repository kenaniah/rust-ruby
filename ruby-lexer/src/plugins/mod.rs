//! Contains plugins that may be used to manipulate the character input or token output streams
//! of the lexer.

mod newline_handler;

pub use newline_handler::NewlinesHandler;
