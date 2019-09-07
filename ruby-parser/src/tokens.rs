/// Ruby source can be tokenized into a sequence of these tokens
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    // 8.1 - Input elements
    EndOfFile,
    // 8.3 - Line terminators
    Separator,      // ;
    LineTerminator, // \n or \r\n
    // 8.4 - Whitespace
    Whitespace, // tab (0x09), vertical tab (0x0b), form feed (0x0c), carriage return (0x0d), space (0x20)
    // 8.5 - Comments
    Comment,
    // 8.6 - End of program markers
    EndOfProgramMarker, // __END__
    // 8.7.2 - Keywords (alphanumerically)
    KwLINE,     // __LINE__
    KwENCODING, // __ENCODING__
    KwFILE,     // __FILE__
    KwBEGIN,    // BEGIN
    KwEND,      // END
    KwAlias,
    KwAnd,
    KwBegin,
    KwBreak,
    KwCase,
    KwClass,
    KwDef,
    KwDefined, // defined?
    KwDo,
    KwElse,
    KwElsif,
    KwEnd,
    KwEnsure,
    KwFor,
    KwFalse,
    KwIf,
    KwIn,
    KwModule,
    KwNext,
    KwNil,
    KwNot,
    KwOr,
    KwRedo,
    KwRescue,
    KwRetry,
    KwReturn,
    KwSelf,
    KwSuper,
    KwThen,
    KwTrue,
    KwUndef,
    KwUnless,
    KwUntil,
    KwWhen,
    KwWhile,
    KwYield,
    // 8.7.3 - Identifiers
    LocalVariableIdentifier,
    GlobalVariableIdentifier,
    ClassVariableIdentifier,
    InstanceVariableIdentifier,
    ConstantIdentifier,
    MethodOnlyIdentifier,
    AssignmentLikeMethodIdentifier,
    // 8.7.4 - Punctuators
    LeftBracket,  // [
    RightBracket, // ]
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    DblColon,     // ::
    Comma,        // ,
    Semicolon,    // ;
    TwoDot,       // ..
    ThreeDot,     // ...
    QuestionMark, // ?
    Colon,        // :
    Arrow,        // =>
    // 8.7.5 - Operators
    OpNot,      // !
    OpNotEqual, // !=
    OpNotMatch, // !~
    OpAnd,      // &&
    OpOr,       // ||
}
