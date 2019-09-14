mod tests {

    use super::super::{make_tokenizer, Token};
    use std::iter::FromIterator;
    use std::iter::Iterator;

    pub fn lex_source(source: &String) -> Vec<Token> {
        let lexer = make_tokenizer(source);
        Vec::from_iter(lexer.map(|x| x.unwrap().1))
    }

    #[test]
    fn test_whitespace() {
        let source = String::from("  \x0c \x0b  \t \t \n\r\n\n\r");
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
        )
    }

    #[test]
    fn test_whitespace_continued_line() {
        let source = String::from("foo\\\r\n\\\nbar \\\n baz\n");
        let tokens = lex_source(&source);
        assert_eq!(
            tokens,
            vec![
                Token::RefactorIdentifier {
                    value: String::from("foo")
                },
                Token::Whitespace,
                Token::RefactorIdentifier {
                    value: String::from("bar")
                },
                Token::Whitespace,
                Token::RefactorIdentifier {
                    value: String::from("baz")
                },
                Token::LineTerminator
            ]
        )
    }

    #[test]
    fn test_whitespace_continued_line_only() {
        let source = String::from("\\\n\n");
        let tokens = lex_source(&source);
        assert_eq!(
            tokens,
            vec![
                Token::Whitespace,
                Token::LineTerminator
            ]
        )
    }

}
