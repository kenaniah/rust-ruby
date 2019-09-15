use super::*;

#[test]
fn single_line_comments() {
    // Test comment only
    let mut source = "#".to_owned();
    let mut tokens = lex_source(&source);
    assert_eq!(
        tokens,
        vec![Token::Comment {
            value: "#".to_owned()
        }]
    );

    // Test a single line comment
    source = "# comment goes here".to_owned();
    tokens = lex_source(&source);
    assert_eq!(
        tokens,
        vec![Token::Comment {
            value: "# comment goes here".to_owned()
        }]
    );

    // Test two single line comments in a row
    source = "# first comment\n#second\tcomment".to_owned();
    tokens = lex_source(&source);
    assert_eq!(
        tokens,
        vec![
            Token::Comment {
                value: "# first comment".to_owned()
            },
            Token::LineTerminator,
            Token::Comment {
                value: "#second\tcomment".to_owned()
            }
        ]
    );

    // Test comments after an expression
    source = "foo# first comment\nbar # second comment\n".to_owned();
    tokens = lex_source(&source);
    assert_eq!(
        tokens,
        vec![
            Token::RefactorIdentifier {
                value: "foo".to_owned()
            },
            Token::Comment {
                value: "# first comment".to_owned()
            },
            Token::LineTerminator,
            Token::RefactorIdentifier {
                value: "bar".to_owned()
            },
            Token::Whitespace,
            Token::Comment {
                value: "# second comment".to_owned()
            },
            Token::LineTerminator
        ]
    );
}
