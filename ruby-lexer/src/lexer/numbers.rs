use super::{LexResult, LexState, Lexer, LexicalError, LexicalErrorType, Token};
//use num_bigint::BigInt;

impl<T> Lexer<T>
where
    T: Iterator<Item = char>,
{
    /// Lexes and returns a numeric token
    pub fn lex_number(&mut self) -> LexResult {
        // parse.y:5052
        let mut seen_point = false;
        let mut seen_e = false;
        let mut non_digit: Option<char> = None;
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
                non_digit = Some(c);
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
                        non_digit = None;
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

        // TODO: 5192 and beyond
        while let Some(c) = self.char(0) {
            // Handle digits
            if c.is_digit(radix) {
                tok.push(self.next_char().unwrap());
                non_digit = None;
                continue;
            }
            // Ignore underscores
            if c == '_' {
                if non_digit != None {
                    break;
                }
                non_digit = self.next_char();
                continue;
            }
            // Handle points in decimal
            if radix == 10 && c == '.' && Self::is_digit(self.char(1), 10) {
                if seen_point || seen_e || non_digit != None {
                    break;
                }
                seen_point = true;
                tok.push(self.next_char().unwrap());
                continue;
            }
            // Handle scientific notation in decimal
            if radix == 10 && (c == 'e' || c == 'E') {
                if seen_e || non_digit != None {
                    break;
                }
                non_digit = self.char(0);
                if Self::is_digit(self.char(1), 10) {
                    seen_e = true;
                    tok.push(self.next_char().unwrap());
                    continue;
                }
                if (self.char(1) == Some('+') || self.char(1) == Some('-')) && Self::is_digit(self.char(2), 10) {
                    seen_e = true;
                    tok.push(self.next_char().unwrap());
                    tok.push(self.next_char().unwrap());
                    continue;
                }
            }
            // Anything else is not part of the number
            break;
        }

        // Error on trailing characters
        if let Some(c) = non_digit {
            return Err(LexicalError {
                message: format!("trailing '{}' in number", c),
                location: self.get_pos(),
                error: LexicalErrorType::LexingError
            });
        }

        // Return a parsed token
        if seen_e || seen_point {
            let value = tok.parse::<f64>().unwrap(); // TODO: handle fails
            return Ok((start, Token::Float { value: value }, self.get_pos()));
        } else {
            let value = tok.parse::<isize>().unwrap(); // TODO: handle fails
            return Ok((start, Token::Integer { value: value}, self.get_pos()));
        }

    }

    /// Wraps char#is_digit
    pub fn is_digit(c: Option<char>, radix: u32) -> bool {
        if let Some(c) = c {
            return c.is_digit(radix);
        }
        false
    }
}
