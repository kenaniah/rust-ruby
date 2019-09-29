//use num_bigint::BigInt;

/// Ruby source can be tokenized into a sequence of these tokens
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
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
    KwDoForCondition, // from ruby's parse.y
    KwDoForBlock,     // from ruby's parse.y
    KwDoForLambda,    // from ruby's parse.y
    KwElse,
    KwElsif,
    KwEnd,
    KwEnsure,
    KwFor,
    KwFalse,
    KwIf,
    KwIfModifier, // from ruby's parse.y
    KwIn,
    KwModule,
    KwNext,
    KwNil,
    KwNot,
    KwOr,
    KwRedo,
    KwRescue,
    KwRescueModifier, // from ruby's parse.y
    KwRetry,
    KwReturn,
    KwSelf,
    KwSuper,
    KwThen,
    KwTrue,
    KwUndef,
    KwUnless,
    KwUnlessModifier, // from ruby's parse.y
    KwUntil,
    KwUntilModifier, // from ruby's parse.y
    KwWhen,
    KwWhile,
    KwWhileModifier, // from ruby's parse.y
    KwYield,
    // Other tokens (for now)
    // 8.1 - Input elements
    EndOfFile,
    // 8.3 - Line terminators
    Separator,      // ;
    Newline, // \n or \r\n, syntatically insignificant
    LineTerminator, // \n or \r\n, syntatically significant
    // 8.4 - Whitespace
    Whitespace, // tab (0x09), vertical tab (0x0b), form feed (0x0c), carriage return (0x0d), space (0x20)
    // 8.5 - Comments
    Comment { value: String },
    // 8.6 - End of program markers
    EndOfProgramMarker, // __END__
    // 8.7.3 - Identifiers
    LocalVariableIdentifier,
    GlobalVariableIdentifier, // tGVAR
    ClassVariableIdentifier, // tCVAR
    InstanceVariableIdentifier, // tIVAR
    ConstantIdentifier,
    MethodOnlyIdentifier,
    AssignmentLikeMethodIdentifier,
    // 8.7.4 - Punctuators
    LeftBracket,   // [
    RightBracket,  // ]
    LeftParen,     // (
    RightParen,    // )
    LeftBrace,     // {
    RightBrace,    // }
    DoubleColon,   // ::
    Comma,         // ,
    Semicolon,     // ;
    TwoDot,        // ..
    ThreeDot,      // ...
    OpTernaryIf,   // ?
    OpTernaryElse, // :
    Arrow,         // => tASSOC
    // 8.7.5 - Operators
    OpNot,      // !
    OpNotEqual, // != tNEQ
    OpNotMatch, // !~ tNMATCH
    OpAnd,      // &&
    OpOr,       // ||
    OpAssign,   // =
    // Operator methods
    OpBinXor,        // ^
    OpBinAnd,        // &
    OpBinOr,         // |
    OpCompare,       // <=> tCMP
    OpDoubleEqual,   // == tEQ
    OpTripleEqual,   // === tEQQ
    OpMatch,         // =~ tMATCH
    OpGt,            // >
    OpGtEqual,       // >= tGEQ
    OpLt,            // <
    OpLtEqual,       // <= tLEQ
    OpLeftShift,     // <<
    OpRightShift,    // >> tRSHFT
    OpPlus,          // +
    OpMinus,         // -
    OpMultiply,      // *
    OpDivide,        // /
    OpModulus,       // %
    OpExponent,      // ** tPOW
    OpBinComplement, // ~
    OpUnaryPlus,     // +@
    OpUnaryMinus,    // -@
    OpElementGet,    // []
    OpElementSet,    // []=
    // Operator assignment methods
    AssignmentOperator { value: String },
    // 8.7.6 - Literals
    Integer { value: isize },
    Float { value: f64 },
    Complex { real: f64, imag: f64 },
    String { value: String },
    Regex { value: String },
    Symbol { value: String },
    // Things that need refactoring down the line
    RefactorIdentifier { value: String },
    Dot, // .
    Star, // * tSTAR
    TwoStar, // ** tDSTAR
}
