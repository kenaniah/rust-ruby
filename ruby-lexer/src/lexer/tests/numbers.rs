use super::*;

#[test]
fn zero_in_different_radixes() {
    enable_logging();

    // Integer representations
    assert_eq!(lex_source("0"), Ok(vec![Token::Integer { value: 0 }]));
    assert_eq!(lex_source("00"), Ok(vec![Token::Integer { value: 0 }]));
    assert_eq!(lex_source("0b0"), Ok(vec![Token::Integer { value: 0 }]));
    assert_eq!(lex_source("0B0"), Ok(vec![Token::Integer { value: 0 }]));
    assert_eq!(lex_source("0o0"), Ok(vec![Token::Integer { value: 0 }]));
    assert_eq!(lex_source("0O0"), Ok(vec![Token::Integer { value: 0 }]));
    assert_eq!(lex_source("0d0"), Ok(vec![Token::Integer { value: 0 }]));
    assert_eq!(lex_source("0D0"), Ok(vec![Token::Integer { value: 0 }]));
    assert_eq!(lex_source("0x0"), Ok(vec![Token::Integer { value: 0 }]));
    assert_eq!(lex_source("0X0"), Ok(vec![Token::Integer { value: 0 }]));

    // Float representations
    assert_eq!(lex_source("0.0"), Ok(vec![Token::Float { value: 0.0 }]));
    assert_eq!(lex_source("0.00"), Ok(vec![Token::Float { value: 0.0 }]));

    // Scientific representations
    assert_eq!(lex_source("0e0"), Ok(vec![Token::Float { value: 0.0 }]));
    assert_eq!(lex_source("0E0"), Ok(vec![Token::Float { value: 0.0 }]));
    assert_eq!(lex_source("0e+0"), Ok(vec![Token::Float { value: 0.0 }]));
    assert_eq!(lex_source("0e-0"), Ok(vec![Token::Float { value: 0.0 }]));
    assert_eq!(lex_source("0E+0"), Ok(vec![Token::Float { value: 0.0 }]));
    assert_eq!(lex_source("0E-0"), Ok(vec![Token::Float { value: 0.0 }]));

}
