use super::*;

#[test]
fn single_line_comments() {
    // Test comment only
    let mut tokens = lex_source("#");
    assert_eq!(
        tokens,
        Ok(vec![Token::Comment {
            value: "".to_owned()
        }])
    );

    // Test a single line comment
    tokens = lex_source("# comment goes here");
    assert_eq!(
        tokens,
        Ok(vec![Token::Comment {
            value: " comment goes here".to_owned()
        }])
    );

    // Test two single line comments in a row
    tokens = lex_source("# first comment\n#second\tcomment");
    assert_eq!(
        tokens,
        Ok(vec![
            Token::Comment {
                value: " first comment".to_owned()
            },
            Token::LineTerminator,
            Token::Comment {
                value: "second\tcomment".to_owned()
            }
        ])
    );

    // Test comments after an expression
    tokens = lex_source("foo# first comment\nbar # second comment\n");
    assert_eq!(
        tokens,
        Ok(vec![
            Token::RefactorIdentifier {
                value: "foo".to_owned()
            },
            Token::Comment {
                value: " first comment".to_owned()
            },
            Token::LineTerminator,
            Token::RefactorIdentifier {
                value: "bar".to_owned()
            },
            Token::Whitespace,
            Token::Comment {
                value: " second comment".to_owned()
            },
            Token::LineTerminator
        ])
    );
}

#[test]
fn multi_line_comments() {
    // Test comment only
    let tokens = lex_source("=begin\nfoo bar\nblah\n=end baz");
    assert_eq!(
        tokens,
        Ok(vec![Token::Comment {
            value: "foo bar\nblah\nbaz".to_owned()
        }])
    )
}
