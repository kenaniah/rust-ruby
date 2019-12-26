use super::{LexResult, LexState, Lexer, Location, SpannedToken, Token, BUFFER_SIZE};
use log::trace;

enum IdentifierType {
    Global,
    Instance,
    Class,
    MethodOnly,
    AssignmentLike,
    Constant,
    Identifier,
}

impl<T> Lexer<T>
where
    T: Iterator<Item = char>,
{
    /// Checks if the given character is a valid Ruby identifier character
    ///
    /// Identifying characters include `[a-zA-Z0-9_]` and non-ascii characters
    pub fn is_identchar(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_' || !c.is_ascii()
    }

    /// Lexes and returns an identifier or a language keyword
    pub fn lex_identifier(&mut self, prefix: String) -> LexResult {
        // Check for the program end token followed by a newline or EOF
        let start_pos = self.get_pos();
        if prefix == "" && start_pos.col() == 1 && self.chars(7) == Some("__END__".to_owned()) {
            if self.char(7) == Some('\n') || self.char(7) == None {
                return self.emit_from_chars(Token::EndOfProgramMarker, 7);
            }
        }

        // Consume any prefix characters (@, @@, $)
        let mut ident = prefix;
        for _ in 0..ident.chars().count() {
            self.next_char();
        }
        // Consume any identifier characters
        while let Some(c) = self.char(0) {
            if Self::is_identchar(c) {
                ident.push(self.next_char().unwrap());
                continue;
            }
            break;
        }
        // Add a method-like identifying character if not followed by '='
        let mut method_only = false;
        if (self.char(0) == Some('!') || self.char(0) == Some('?')) && self.char(1) != Some('=') {
            ident.push(self.next_char().unwrap());
            method_only = true;
        }

        let token_type: IdentifierType = match ident.chars().nth(0).unwrap() {
            '$' => {
                self.lex_state = LexState::EXPR_END;
                IdentifierType::Global
            }
            '@' => {
                if ident.chars().nth(1) == Some('@') {
                    IdentifierType::Class
                } else {
                    IdentifierType::Instance
                }
            }
            _ => {
                if method_only {
                    IdentifierType::MethodOnly
                } else {
                    let result = if self.lex_state == LexState::EXPR_FNAME
                        && self.char(0) == Some('=')
                        && self.char(1) != Some('~')
                        && self.char(1) != Some('>')
                        && (self.char(1) != Some('=') || self.char(2) == Some('>'))
                    {
                        ident.push(self.next_char().unwrap());
                        IdentifierType::AssignmentLike
                    } else if ident.chars().nth(0).unwrap().is_ascii_uppercase() {
                        IdentifierType::Constant
                    } else {
                        IdentifierType::Identifier
                    };
                    // Check for a label
                    if self.is_label_possible() && self.is_label_suffix(0) {
                        self.lex_state = LexState::EXPR_END;
                        return Ok((
                            start_pos,
                            Token::Identifier { value: ident },
                            self.get_pos(),
                        ));
                    }
                    // Check for a keyword
                    if self.lex_state != LexState::EXPR_DOT {
                        // TODO: parse.y:5749
                        todo!()
                    }
                    // Update the lexer's state
                    self.lex_state =
                        if self.is_beg() || self.lex_state == LexState::EXPR_DOT || self.is_arg() {
                            if self.prev_command_state {
                                LexState::EXPR_CMDARG
                            } else {
                                LexState::EXPR_ARG
                            }
                        } else if self.lex_state == LexState::EXPR_FNAME {
                            LexState::EXPR_ENDFN
                        } else {
                            LexState::EXPR_END
                        };
                    // TODO: parse.y:5807
                    result
                }
            }
        };

        // parse.y:5679

        todo!()
    }

    // Lexes a named identifier
    // fn lex_identifier(&mut self) -> LexResult {
    //     let mut name = String::new();
    //     let start_pos = self.get_pos();
    //
    //     // Take the first character
    //     name.push(self.next_char().unwrap());
    //
    //     // Check for more identifier characters
    //     while self.is_identifier_continuation() {
    //         name.push(self.next_char().unwrap());
    //     }
    //
    //     // Check for an ending ? or ! (valid for method names)
    //     if self.char(0) == Some('?') || self.char(0) == Some('!') {
    //         name.push(self.next_char().unwrap());
    //     }
    //
    //     let end_pos = self.get_pos();
    //
    //     // Emit the token
    //     if self.keywords.contains_key(&name) {
    //         Ok((start_pos, self.keywords[&name].clone(), end_pos))
    //     } else {
    //         Ok((
    //             start_pos,
    //             Token::RefactorIdentifier { value: name },
    //             end_pos,
    //         ))
    //     }
    // }
}
