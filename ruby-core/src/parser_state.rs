use crate::RubyState;

/**
Implementation of `mrb_parser_state`
*/
pub struct ParserState<'a> {
    /// Ruby's current state (`mrb_state *mrb`)
    pub(crate) ruby_state: &'a RubyState
}
