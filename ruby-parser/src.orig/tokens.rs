use num_bigint::BigInt;

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
    Comment { value: String },
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
    Arrow,         // =>
    // 8.7.5 - Operators
    OpNot,      // !
    OpNotEqual, // !=
    OpNotMatch, // !~
    OpAnd,      // &&
    OpOr,       // ||
    OpAssign,   // =
    // Operator methods
    OpBinXor,        // ^
    OpBinAnd,        // &
    OpBinOr,         // |
    OpCompare,       // <=>
    OpDoubleEqual,   // ==
    OpTripleEqual,   // ===
    OpMatch,         // =~
    OpGt,            // >
    OpGtEqual,       // >=
    OpLt,            // <
    OpLtEqual,       // <=
    OpLeftShift,     // <<
    OpRightShift,    // >>
    OpPlus,          // +
    OpMinus,         // -
    OpMultiply,      // *
    OpDivide,        // /
    OpModulus,       // %
    OpExponent,      // **
    OpBinComplement, // ~
    OpUnaryPlus,     // +@
    OpUnaryMinus,    // -@
    OpElementGet,    // []
    OpElementSet,    // []=
    // Operator assignment methods
    OpAndAssign,        // &&=
    OpOrAssign,         // ||=
    OpXorAssign,        // ^=
    OpBinAndAssign,     // &=
    OpBinOrAssign,      // |=
    OpLeftShiftAssign,  // <<=
    OpRightShiftAssign, // >>=
    OpPlusAssign,       // +=
    OpMinusAssign,      // -=
    OpMultiplyAssign,   // *=
    OpDivideAssign,     // /=
    OpModulusAssign,    // %=
    OpExponentAssign,   // **=
    // 8.7.6 - Literals
    Integer { value: BigInt },
    Float { value: f64 },
    Complex { real: f64, imag: f64 },
    String { value: String },
    Regex { value: String },
    Symbol { value: String },
    // Things that need refactoring down the line
    RefactorIdentifier { value: String },
}