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
    KwDoForCondition, // keyword_do_cond
    KwDoForBlock,     // keyword_do_block
    KwDoForLambda,    // keyword_do_LAMBDA
    KwElse,
    KwElsif,
    KwEnd,
    KwEnsure,
    KwFor,
    KwFalse,
    KwIf,
    KwIfModifier, // modifier_if
    KwIn,
    KwModule,
    KwNext,
    KwNil,
    KwNot,
    KwOr,
    KwRedo,
    KwRescue,
    KwRescueModifier, // modifier_rescue
    KwRetry,
    KwReturn,
    KwSelf,
    KwSuper,
    KwThen,
    KwTrue,
    KwUndef,
    KwUnless,
    KwUnlessModifier, // modifier_unless
    KwUntil,
    KwUntilModifier, // modifier_until
    KwWhen,
    KwWhile,
    KwWhileModifier, // modifier_while
    KwYield,
    // Other tokens (for now)
    // 8.1 - Input elements
    EndOfFile,
    // 8.3 - Line terminators
    Separator,      // ;
    Newline,        // \n or \r\n, syntatically insignificant
    LineTerminator, // \n or \r\n, syntatically significant
    // 8.4 - Whitespace
    Whitespace, // tab (0x09), vertical tab (0x0b), form feed (0x0c), carriage return (0x0d), space (0x20)
    // 8.5 - Comments
    Comment { value: String },
    // 8.6 - End of program markers
    EndOfProgramMarker, // __END__
    // 8.7.3 - Identifiers
    /// **Original Grammar:** `tIDENTIFIER`
    Identifier { value: String },
    /// **Original Grammar:** `tGVAR`
    GlobalVariable { value: String },
    /// **Original Grammar:** `tCVAR`
    ClassVariable { value: String },
    /// **Original Grammar:** `tIVAR`
    InstanceVariable { value: String },
    /// **Original Grammar:** `tCONSTANT`
    Constant { value: String },
    /// **Original Grammar:** `tFID`
    FunctionIdentifier { value: String },
    /// **Original Grammar:** `tLABEL_TAG`
    LabelTag { value: String },
    AssignmentLikeMethodIdentifier { value: String },
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
    /// **Original Grammar:** `tINTEGER`
    Integer { value: isize },
    /// **Original Grammar:** `tFLOAT`
    Float { value: f64 },
    /// **Original Grammar:** `tCHAR`
    ///
    /// Represents the `?<char>` character literal notation that can be used to build single character strings.
    /// Valid forms include:
    /// * `?x` - where `x` represents any unescaped ASCII character or any valid character escape sequence
    /// * `?cx` - Control + `x`
    /// * `?\C-x` - Control + `x`
    /// * `?\M-x` - Meta + `x`
    /// * `?\C-\M-x` - Control + Meta + `x`
    /// * `?\M-\C-x` - Control + Meta + `x`
    ///
    /// See [Ruby's string literal syntax](https://github.com/ruby/ruby/blob/trunk/doc/syntax/literals.rdoc#strings) for more info.
    Char { value: String },
    Complex { real: f64, imag: f64 },
    /// **Original Grammar:** `tXSTRING`
    ///
    /// Represents a backtick string (which captures the result of a subshell). Backtick strings come in two forms:
    /// * `` `backtick string` ``
    /// * `%{backtick string}`
    XString { value: String },
    /// **Original Grammar:** `tSTRING`
    String { value: String },
    /// **Original Grammar:** `tSTRING_PART`
    StringPart { value: String },
    /// **Original Grammar:** `tSTRING_MID`
    StringMid { value: String },
    /// **Original Grammar:** `tREGEXP`
    Regex { value: String },
    /// **Original Grammar:** `tNTH_REF`
    RegexNthRef { value: String },
    /// **Original Grammar:** `tBACK_REF`
    RegexBackRef { value: String },
    Symbol { value: String },
    // Things that need refactoring down the line
    RefactorIdentifier { value: String },
    // Character tokens
    At,        // @
    Dot,       // .
    Star,      // * tSTAR
    TwoStar,   // ** tDSTAR
    Backslash, // \
}
