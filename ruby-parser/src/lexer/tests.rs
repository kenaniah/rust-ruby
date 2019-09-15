use super::{make_tokenizer, LexicalError, Token};

/// Lexes the source string, returning a vector of tokens or the lexical error encountered
pub fn lex_source(source: &str) -> Result<Vec<Token>, LexicalError> {
    let mut lexer = make_tokenizer(source);
    let mut tokens: Vec<Token> = Vec::new();

    // Move through the lexer, returning a lexical error if encountered
    while let Some(x) = lexer.next() {
        match x {
            Ok(spanned) => tokens.push(spanned.1),
            Err(error) => return Err(error),
        }
    }

    // Return the lexed tokens
    Ok(tokens)
}

// Include the various test suites
mod comment;
mod whitespace;
