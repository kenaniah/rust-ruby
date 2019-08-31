#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub calc);

#[cfg(test)]
mod tests {
    lalrpop_mod!(pub calc);
    #[test]
    fn it_works() {
        assert!(calc::TermParser::new().parse("22").is_ok());
        assert!(calc::TermParser::new().parse("foo").is_err());
    }
    #[test]
    fn second() {
        assert!(calc::TermParser::new().parse("(44)").is_ok());
        assert!(calc::TermParser::new().parse("foo").is_err());
    }
}
