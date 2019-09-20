//! Provides the constants and enums found in CRuby's implementation for working with ruby values
//! See https://github.com/ruby/ruby/blob/master/include/ruby/ruby.h

#[cfg(target_pointer_width = "64")]
bitflags! {
    pub struct ValueFlags: u32 {

        // Quick value constants
        const Q_FALSE = 0b0000_0000; // 0x00
        const Q_TRUE  = 0b0001_0100; // 0x14
        const Q_NIL   = 0b0000_1000; // 0x08
        const Q_UNDEF = 0b0011_0100; // 0x34

        // Value type masks
        const IMMEDIATE_MASK = 0b0000_0111; // 0x07
        const FIXNUM_FLAG    = 0b0000_0001; // 0x01
        const FLONUM_MASK    = 0b0000_0011; // 0x03
        const FLONUM_FLAG    = 0b0000_0010; // 0x02
        const SYMBOL_FLAG    = 0b0000_1100; // 0x0c

    }
}

#[cfg(target_pointer_width = "32")]
bitflags! {
    pub struct ValueFlags: u32 {

        // Quick value constants
        const Q_FALSE = 0b0000_0000; // 0x00
        const Q_TRUE  = 0b0000_0010; // 0x02
        const Q_NIL   = 0b0000_0100; // 0x04
        const Q_UNDEF = 0b0000_0110; // 0x06

        // Value type masks
        const IMMEDIATE_MASK = 0b0000_0011; // 0x03
        const FIXNUM_FLAG    = 0b0000_0001; // 0x01
        const FLONUM_MASK    = 0b0000_0000; // 0x00 (any values ANDed with FLONUM_MASK can not be FLONUM_FLAG)
        const FLONUM_FLAG    = 0b0000_0010; // 0x02
        const SYMBOL_FLAG    = 0b0000_1110; // 0x0e

    }
}

pub enum ValueType {
    None = 0x00,
    Object = 0x01,
    Class = 0x02,
    Module = 0x03,
    Float = 0x04,
    String = 0x05,
    Regexp = 0x06,
    Array = 0x07,
    Hash = 0x08,
    Struct = 0x09,
    Bignum = 0x0a,
    File = 0x0b,
    Data = 0x0c,
    Match = 0x0d,
    Complex = 0x0e,
    Rational = 0x0f,
    Nil = 0x11,
    True = 0x12,
    False = 0x13,
    Symbol = 0x14,
    Fixnum = 0x15,
    Undef = 0x16,
    Imemo = 0x1a,
    Node = 0x1b,
    Iclass = 0x1c,
    Zombie = 0x1d,
    TypeMask = 0x1f,
}

bitflags! {
    pub struct LexStateBits: u32 {
        const EXPR_BEG = 1 << 0; /* ignore newline, +/- is a sign. */
        const EXPR_END = 1 << 1; /* newline significant, +/- is an operator. */
        const EXPR_ENDARG = 1 << 2; /* ditto, and unbound braces. */
        const EXPR_ENDFN = 1 << 3; /* ditto, and unbound braces. */
        const EXPR_ARG = 1 << 4; /* newline significant, +/- is an operator. */
        const EXPR_CMDARG = 1 << 5; /* newline significant, +/- is an operator. */
        const EXPR_MID = 1 << 6; /* newline significant, +/- is an operator. */
        const EXPR_FNAME = 1 << 7; /* ignore newline, no reserved words. */
        const EXPR_DOT = 1 << 8; /* right after `.' or `::', no reserved words. */
        const EXPR_CLASS = 1 << 9; /* immediate after `class', no here document. */
        const EXPR_LABEL = 1 << 10; /* flag bit, label is allowed. */
        const EXPR_LABELED = 1 << 11; /* flag bit, just after a label. */
        const EXPR_FITEM = 1 << 12; /* symbol literal as FNAME. */
        const EXPR_MAX_STATE = 1 << 13;
    }
}
