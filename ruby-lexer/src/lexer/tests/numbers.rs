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

#[test]
fn scientific_notations() {
    enable_logging();
    assert_eq!(
        lex_source("24e+3"),
        Ok(vec![Token::Float { value: 24000.0 }])
    );
    assert_eq!(
        lex_source("2.4E+2"),
        Ok(vec![Token::Float { value: 240.0 }])
    );
    assert_eq!(
        lex_source("-52.8e-1"),
        Ok(vec![Token::Float { value: -5.28 }])
    );
    assert_eq!(
        lex_source("+0.00528e12"),
        Ok(vec![Token::Float {
            value: 5280000000.0
        }])
    );
}

#[test]
fn binary() {
    enable_logging();
    assert_eq!(lex_source("0b01"), Ok(vec![Token::Integer { value: 1 }]));
    assert_eq!(lex_source("0b10"), Ok(vec![Token::Integer { value: 2 }]));
    assert_eq!(
        lex_source("0b1000_0101"),
        Ok(vec![Token::Integer { value: 133 }])
    );
}

#[test]
fn octal() {
    enable_logging();
    assert_eq!(lex_source("001"), Ok(vec![Token::Integer { value: 1 }]));
    assert_eq!(lex_source("020"), Ok(vec![Token::Integer { value: 16 }]));
    assert_eq!(lex_source("0o11"), Ok(vec![Token::Integer { value: 9 }]));
    assert_eq!(lex_source("+0O12"), Ok(vec![Token::Integer { value: 10 }]));
    assert_eq!(lex_source("0_13"), Ok(vec![Token::Integer { value: 11 }]));
    assert_eq!(
        lex_source("-0_1_7"),
        Ok(vec![Token::Integer { value: -15 }])
    );
}

#[test]
fn decimal() {
    enable_logging();
    assert_eq!(
        lex_source("0d12345"),
        Ok(vec![Token::Integer { value: 12345 }])
    );
    assert_eq!(lex_source("-0D99"), Ok(vec![Token::Integer { value: -99 }]));
    assert_eq!(lex_source("12.03"), Ok(vec![Token::Float { value: 12.03 }]));
    assert_eq!(lex_source("-8.000"), Ok(vec![Token::Float { value: -8.0 }]));
    assert_eq!(lex_source("+4"), Ok(vec![Token::Integer { value: 4 }]));
    assert_eq!(
        lex_source("-4_000"),
        Ok(vec![Token::Integer { value: -4000 }])
    );
}

#[test]
fn hexadecimal() {
    enable_logging();
    assert_eq!(lex_source("0xA"), Ok(vec![Token::Integer { value: 10 }]));
    assert_eq!(lex_source("0xE2"), Ok(vec![Token::Integer { value: 226 }]));
    assert_eq!(
        lex_source("-0xFb"),
        Ok(vec![Token::Integer { value: -251 }])
    );
    assert_eq!(
        lex_source("+0x2921_B4"),
        Ok(vec![Token::Integer { value: 2695604 }])
    );
}

// TODO
// test max & min for floats and integers
// test bigint stuff
// test lexical errors
