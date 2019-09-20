bitflags! {
    #[derive(Default)]
    pub struct LexState: u32 {
        const EXPR_NONE         = 0;
        const EXPR_BEG          = 1 << 0; /* ignore newline, +/- is a sign. */
        const EXPR_END          = 1 << 1; /* newline significant, +/- is an operator. */
        const EXPR_ENDARG       = 1 << 2; /* ditto, and unbound braces. */
        const EXPR_ENDFN        = 1 << 3; /* ditto, and unbound braces. */
        const EXPR_ARG          = 1 << 4; /* newline significant, +/- is an operator. */
        const EXPR_CMDARG       = 1 << 5; /* newline significant, +/- is an operator. */
        const EXPR_MID          = 1 << 6; /* newline significant, +/- is an operator. */
        const EXPR_FNAME        = 1 << 7; /* ignore newline, no reserved words. */
        const EXPR_DOT          = 1 << 8; /* right after `.' or `::', no reserved words. */
        const EXPR_CLASS        = 1 << 9; /* immediate after `class', no here document. */
        const EXPR_LABEL        = 1 << 10; /* flag bit, label is allowed. */
        const EXPR_LABELED      = 1 << 11; /* flag bit, just after a label. */
        const EXPR_FITEM        = 1 << 12; /* symbol literal as FNAME. */
        const EXPR_MAX_STATE    = 1 << 13;
        const EXPR_VALUE        = (Self::EXPR_BEG.bits() | Self::EXPR_MID.bits() | Self::EXPR_CLASS.bits());
        const EXPR_ARG_ANY      = (Self::EXPR_ARG.bits() | Self::EXPR_CMDARG.bits());
        const EXPR_END_ANY      = (Self::EXPR_END.bits() | Self::EXPR_ENDARG.bits() | Self::EXPR_ENDFN.bits());
    }
}
