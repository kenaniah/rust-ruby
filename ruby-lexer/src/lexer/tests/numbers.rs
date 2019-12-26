use super::*;

fn integer(value: isize) -> Result<Vec<Token>, LexicalError> {
    Ok(vec![Token::Integer { value: value }])
}

fn float(value: f64) -> Result<Vec<Token>, LexicalError> {
    Ok(vec![Token::Float { value: value }])
}

#[test]
fn zero_in_different_radixes() {
    enable_logging();

    // Integer representations
    assert_eq!(lex_source("0"), integer(0));
    assert_eq!(lex_source("00"), integer(0));
    assert_eq!(lex_source("0b0"), integer(0));
    assert_eq!(lex_source("0B0"), integer(0));
    assert_eq!(lex_source("0o0"), integer(0));
    assert_eq!(lex_source("0O0"), integer(0));
    assert_eq!(lex_source("0d0"), integer(0));
    assert_eq!(lex_source("0D0"), integer(0));
    assert_eq!(lex_source("0x0"), integer(0));
    assert_eq!(lex_source("0X0"), integer(0));

    // Float representations
    assert_eq!(lex_source("0.0"), float(0.0));
    assert_eq!(lex_source("0.00"), float(0.0));

    // Scientific representations
    assert_eq!(lex_source("0e0"), float(0.0));
    assert_eq!(lex_source("0E0"), float(0.0));
    assert_eq!(lex_source("0e+0"), float(0.0));
    assert_eq!(lex_source("0e-0"), float(0.0));
    assert_eq!(lex_source("0E+0"), float(0.0));
    assert_eq!(lex_source("0E-0"), float(0.0));
}

#[test]
fn scientific_notations() {
    enable_logging();
    assert_eq!(lex_source("24e+3"), float(24000.0));
    assert_eq!(lex_source("2.4E+2"), float(240.0));
    assert_eq!(lex_source("-52.8e-1"), float(-5.28));
    assert_eq!(lex_source("+0.00528e12"), float(5280000000.0));
}

#[test]
fn binary() {
    enable_logging();
    assert_eq!(lex_source("0b01"), integer(1));
    assert_eq!(lex_source("0b10"), integer(2));
    assert_eq!(lex_source("0b1000_0101"), integer(133));
}

#[test]
fn octal() {
    enable_logging();
    assert_eq!(lex_source("001"), integer(1));
    assert_eq!(lex_source("020"), integer(16));
    assert_eq!(lex_source("0o11"), integer(9));
    assert_eq!(lex_source("+0O12"), integer(10));
    assert_eq!(lex_source("0_13"), integer(11));
    assert_eq!(lex_source("-0_1_7"), integer(-15));
}

#[test]
fn decimal() {
    enable_logging();
    assert_eq!(lex_source("0d12345"), integer(12345));
    assert_eq!(lex_source("-0D99"), integer(-99));
    assert_eq!(lex_source("12.03"), float(12.03));
    assert_eq!(lex_source("-8.000"), float(-8.0));
    assert_eq!(lex_source("+4"), integer(4));
    assert_eq!(lex_source("-4_000"), integer(-4000));
}

#[test]
fn hexadecimal() {
    enable_logging();
    assert_eq!(lex_source("0xA"), integer(10));
    assert_eq!(lex_source("0xE2"), integer(226));
    assert_eq!(lex_source("-0xFb"), integer(-251));
    assert_eq!(lex_source("+0x2921_B4"), integer(2695604));
}

// TODO
// test max & min for floats and integers
// test bigint stuff
// test lexical errors
