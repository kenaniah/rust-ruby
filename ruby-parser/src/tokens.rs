/// Ruby source can be tokenized into a sequence of these tokens
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    LineTerminator, // 8.3 - \n or \r\n
    Whitespace,

    // 8.7.2 - Keywords (alphanumerically)
    LINE,     // __LINE__
    ENCODING, // __ENCODING__
    FILE,     // __FILE__
    BEGIN,    // BEGIN
    END,      // END
    Alias,
    And,
    Begin,
    Break,
    Case,
    Class,
    Def,
    Defined, // defined?
    Do,
    Else,
    Elsif,
    End,
    Ensure,
    For,
    False,
    If,
    In,
    Module,
    Next,
    Nil,
    Not,
    Or,
    Redo,
    Rescue,
    Retry,
    Return,
    Slf, // self
    Super,
    Then,
    True,
    Undef,
    Unless,
    Until,
    When,
    While,
    Yield,
}
