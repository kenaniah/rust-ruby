use super::*;

#[test]
fn whitespace_and_newlines() {
    let source = "  \x0c \x0b  \t \t \n\r\n\n\r".to_owned();
    let tokens = lex_source(&source);
    assert_eq!(
        tokens,
        vec![
            Token::Whitespace,
            Token::LineTerminator,
            Token::LineTerminator,
            Token::LineTerminator,
            Token::Whitespace
        ]
    );
}

#[test]
fn continued_lines() {
    let source = "foo\\\r\n\\\nbar \\\n baz\n".to_owned();
    let tokens = lex_source(&source);
    assert_eq!(
        tokens,
        vec![
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
        ]
    );
}

#[test]
fn continued_line_only() {
    let source = "\\\n\n".to_owned();
    let tokens = lex_source(&source);
    assert_eq!(tokens, vec![Token::Whitespace, Token::LineTerminator]);
}
