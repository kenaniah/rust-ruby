#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub ruby26);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert!(ruby26::TermParser::new().parse("22").is_ok());
        assert!(ruby26::TermParser::new().parse("foo").is_err());
    }
    #[test]
    fn second() {
        assert!(ruby26::TermParser::new().parse("(44)").is_ok());
        assert!(ruby26::TermParser::new().parse("foo").is_err());
    }
}
