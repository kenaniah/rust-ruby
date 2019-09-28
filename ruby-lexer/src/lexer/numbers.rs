use super::{LexResult, LexState, Lexer, LexicalError, LexicalErrorType, Token};
//use num_bigint::BigInt;

impl<T> Lexer<T>
where
    T: Iterator<Item = char>,
{
    /// Lexes and returns a numeric token
    pub fn lex_number(&mut self) -> LexResult {
        // parse.y:5052
        let mut is_float = false;
        let mut seen_point = false;
        let mut seen_e = false;
        let mut non_digit = false;
        let mut tok = String::new();
        let start = self.get_pos();
        self.lex_state = LexState::EXPR_ENDARG;

        // Check for signed numbers
        if self.char(0) == Some('-') || self.char(0) == Some('+') {
            tok.push(self.next_char().unwrap());
        }

        let mut radix: u32 = 10;

        // Handle numeric prefixes
        if self.char(0) == Some('0') {
            self.next_char();
            if let Some(c) = self.char(0) {
                non_digit = true;
                let radix = match c {
                    'x' | 'X' => {
                        self.next_char();
                        16
                    }
                    'b' | 'B' => {
                        self.next_char();
                        2
                    }
                    'd' | 'D' => {
                        self.next_char();
                        10
                    }
                    'o' | 'O' => {
                        self.next_char();
                        8
                    }
                    '_' => {
                        self.next_char();
                        8
                    }
                    '0'..='7' => {
                        // This is an octal without a prefix character
                        non_digit = false;
                        8
                    }
                    '8'..='9' => {
                        return Err(LexicalError{
                            message: "Invalid octal digit".to_owned(),
                            location: self.get_pos(),
                            error: LexicalErrorType::LexingError
                        })
                    }
                    '.' |  'e' | 'E' => {
                        tok.push('0');
                        10
                    }
                    _ => {
                        // Only character seen was a zero
                        return Ok((start, Token::Integer { value: 0 }, self.get_pos()));
                    }
                };
            }
        }

        unimplemented!()
    }

    /// Wraps char#is_digit
    pub fn is_digit(c: Option<char>, radix: u32) -> bool {
        if let Some(c) = c {
            return c.is_digit(radix);
        }
        false
    }
}
