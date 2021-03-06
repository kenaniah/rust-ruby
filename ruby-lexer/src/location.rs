// This module defines the struct that will be used by the LALRPOP lexer to track the
// start and end location of tokens within the input.

use std::fmt;

/// Struct used to track the line and column numbers of lexed tokens
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Location {
    pub line: usize,
    pub col: usize,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line {} column {}", self.line, self.col)
    }
}

impl Location {
    pub fn new(line: usize, col: usize) -> Self {
        Location { line, col }
    }
    pub fn line(&self) -> usize {
        self.line
    }
    pub fn col(&self) -> usize {
        self.col
    }
    pub fn reset(&mut self) {
        self.line = 1;
        self.col = 1;
    }
    pub fn move_right(&mut self) {
        self.col += 1;
    }
    pub fn newline(&mut self) {
        self.line += 1;
        self.col = 1;
    }
}
