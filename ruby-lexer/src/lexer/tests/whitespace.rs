use super::*;

#[test]
fn whitespace_and_newlines() {
    let tokens = lex_source("  \x0c \x0b  \t \t \n\r\n\n\r");
    assert_eq!(
        tokens,
        Ok(vec![
            Token::Whitespace,
            Token::Newline,
            Token::Newline,
            Token::Newline,
            Token::Whitespace
        ])
    );
}

#[test]
fn continued_lines() {
    enable_logging();
    let tokens = lex_source("foo\\\r\n\\\nbar \\\n baz\n");
    assert_eq!(
        tokens,
        Ok(vec![
            Token::Identifier {
                value: "foo".to_owned()
            },
            Token::Whitespace,
            Token::Identifier {
                value: "bar".to_owned()
            },
            Token::Whitespace,
            Token::Identifier {
                value: "baz".to_owned()
            },
            Token::Newline
        ])
    );
}

#[test]
fn continued_line_only() {
    let tokens = lex_source("\\\n\n");
    assert_eq!(tokens, Ok(vec![Token::Whitespace, Token::Newline]));
}
