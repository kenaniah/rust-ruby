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
}
