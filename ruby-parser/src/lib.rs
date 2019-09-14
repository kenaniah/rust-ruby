#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub ruby26);
pub mod error;
pub mod lexer;
pub mod location;
pub mod tokens;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert!(ruby26::InputElementParser::new().parse("foo").is_ok());
        assert!(ruby26::InputElementParser::new().parse("`foo").is_err());
    }
    #[test]
    fn second() {
        assert!(ruby26::InputElementParser::new().parse("foo\n").is_ok());
        assert!(ruby26::InputElementParser::new().parse("\r\n__END__asldfjdsalfk").is_ok());
    }
}
