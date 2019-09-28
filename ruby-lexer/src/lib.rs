//! This crate provides a library for lexing Ruby source code.
//!
//! The lexer is an adaptation of [mruby's parse.y](https://github.com/mruby/mruby/blob/2.0.1/mrbgems/mruby-compiler/core/parse.y),
//! which is compatible with Ruby's 2.x syntax.
//!
//! # Usage
//! ...
//!
//! # Example: Lex a Ruby source file with Windows-style line endings
//! ...
//!
//! # Example: Remove comment tokens from the produced token stream
//! ...
//!
//! # Example: Output syntax errors that occurred during lexing
//! ...
//!
//! # Features
//! Talk about LALRPOP integration

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
pub type SpannedToken = (Location, Token, Location);

/// Type used to track the success of a lexing operation
pub type LexResult = Result<SpannedToken, LexicalError>;
