#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub ruby26);
pub mod tokens;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert!("\n" == "\x0a");
        assert!(ruby26::TermParser::new().parse("foo").is_err());
    }
    #[test]
    fn second() {
        assert!(ruby26::TermParser::new().parse("(44)").is_ok());
        assert!(ruby26::TermParser::new().parse("foo").is_err());
    }
}
