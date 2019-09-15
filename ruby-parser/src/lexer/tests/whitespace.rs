use super::*;

#[test]
fn whitespace_and_newlines() {
    let tokens = lex_source("  \x0c \x0b  \t \t \n\r\n\n\r".to_owned());
    assert_eq!(
        tokens,
        Ok(vec![
            Token::Whitespace,
            Token::LineTerminator,
            Token::LineTerminator,
            Token::LineTerminator,
            Token::Whitespace
        ])
    );
}

#[test]
fn continued_lines() {
    let tokens = lex_source("foo\\\r\n\\\nbar \\\n baz\n".to_owned());
    assert_eq!(
        tokens,
        Ok(vec![
            Token::RefactorIdentifier {
                value: "foo".to_owned()
            },
            Token::Whitespace,
            Token::RefactorIdentifier {
                value: "bar".to_owned()
            },
            Token::Whitespace,
            Token::RefactorIdentifier {
                value: "baz".to_owned()
            },
            Token::LineTerminator
        ])
    );
}

#[test]
fn continued_line_only() {
    let tokens = lex_source("\\\n\n".to_owned());
    assert_eq!(tokens, Ok(vec![Token::Whitespace, Token::LineTerminator]));
}
