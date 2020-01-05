use crate::{MemoryPool, RClass, Ruby, Symbol};

#[cfg(feature = "stdio")]
use std::fs::File;

/// Represents the state of the Ruby parser
#[maps_to(mruby: mrb_parser_state)]
pub struct ParserState<'a> {
    /// Ruby's current state (`mrb_state *mrb`)
    rb: &'a Ruby,
    /// Ruby's memory pool (`struct mrb_pool *pool`)
    ruby_pool: &'a MemoryPool<'a>,
    /// (`mrb_ast_node *cells`)
    cells: &'a ASTNode<'a>,
    // const char *s, *send;
    s: u8,
    send: u8,
    // FILE *f;
    #[cfg(feature = "stdio")]
    file: &'a File,
    /// mrbc_context *cxt;
    context: &'a CompileContext<'a>,
    /// mrb_sym filename_sym;
    filename_sym: Symbol,
    /// uint16_t lineno;
    line_no: u16,
    /// int column;
    column: usize,
    /// enum mrb_lex_state_enum lstate;
    lex_state: LexState,
    /// mrb_ast_node *lex_strterm; /* (type nest_level beg . end) */
    lex_strterm: &'a ASTNode<'a>,
    /// unsigned int cond_stack;
    cond_stack: usize,
    /// unsigned int cmdarg_stack;
    cmdarg_stack: usize,
    /// int paren_nest;
    paren_nest: isize,
    /// int lpar_beg;
    lpar_beg: isize,
    /// int in_def;
    in_def: isize,
    /// int in_single;
    in_single: isize,
    /// mrb_bool cmd_start:1;
    cmd_start: bool,
    /// mrb_ast_node *locals;
    locals: &'a ASTNode<'a>,
    /// mrb_ast_node *pb;
    pb: &'a ASTNode<'a>,
    /// char *tokbuf;
    tokbuf: &'a str,
    /// char buf[MRB_PARSER_TOKBUF_SIZE];
    buf: &'a str,
    /// int tidx;
    tidx: usize,
    /// int tsiz;
    tsiz: usize,

    /// mrb_ast_node *all_heredocs; /* list of mrb_parser_heredoc_info* */
    all_heredocs: &'a ASTNode<'a>,
    // mrb_ast_node *heredocs_from_nextline;
    heredocs_from_nextline: &'a ASTNode<'a>,
    // mrb_ast_node *parsing_heredoc;
    parsing_heredoc: &'a ASTNode<'a>,
    // mrb_ast_node *lex_strterm_before_heredoc;
    lex_strterm_before_heredoc: &'a ASTNode<'a>,

    /// void *ylval;
    ylval: (),

    //
    // size_t nerr;
    // size_t nwarn;
    // mrb_ast_node *tree;
    //
    // mrb_bool no_optimize:1;
    // mrb_bool on_eval:1;
    // mrb_bool capture_errors:1;
    // struct mrb_parser_message error_buffer[10];
    // struct mrb_parser_message warn_buffer[10];
    //
    // mrb_sym* filename_table;
    // uint16_t filename_table_length;
    // uint16_t current_filename_index;
    //
    // struct mrb_jmpbuf* jmp;
}

/// Used to track file / line information for AST nodes
#[maps_to(mruby: mrb_ast_node)]
pub struct ASTNode<'a> {
    prev: &'a Self,
    next: &'a Self,
    line_no: u16,
    filename_idx: u16,
}

/// Tracks the load context of the parser
#[maps_to(mruby: mrbc_context)]
pub struct CompileContext<'a> {
    // mrb_sym *syms;
    symbols: &'a Symbol,
    // int slen;
    slen: usize,
    // char *filename;
    filename: &'a usize,
    // uint16_t lineno;
    line_no: u16,
    // int (*partial_hook)(struct mrb_parser_state*);
    partial_hook: &'a fn(&'a ParserState) -> usize,
    // void *partial_data;
    partial_data: &'a u8,
    // struct RClass *target_class;
    target_class: &'a RClass<'a>,
    // mrb_bool capture_errors:1;
    capture_errors: bool,
    // mrb_bool dump_result:1;
    dump_result: bool,
    // mrb_bool no_exec:1;
    no_exec: bool,
    // mrb_bool keep_lv:1;
    keep_lv: bool,
    // mrb_bool no_optimize:1;
    no_optimize: bool,
    // mrb_bool on_eval:1;
    on_eval: bool,
    // size_t parser_nerr;
    parser_err_no: usize,
}

#[allow(non_camel_case_types)]
#[maps_to(mruby: mrb_lex_state_enum)]
/// Represents the current lexing state of the Ruby parser
pub enum LexState {
    EXPR_BEG,    /* ignore newline, +/- is a sign. */
    EXPR_END,    /* newline significant, +/- is an operator. */
    EXPR_ENDARG, /* ditto, and unbound braces. */
    EXPR_ENDFN,  /* ditto, and unbound braces. */
    EXPR_ARG,    /* newline significant, +/- is an operator. */
    EXPR_CMDARG, /* newline significant, +/- is an operator. */
    EXPR_MID,    /* newline significant, +/- is an operator. */
    EXPR_FNAME,  /* ignore newline, no reserved words. */
    EXPR_DOT,    /* right after '.' or '::', no reserved words. */
    EXPR_CLASS,  /* immediate after 'class', no here document. */
    EXPR_VALUE,  /* alike EXPR_BEG but label is disallowed. */
}

#[maps_to(mruby: mrb_parser_message)]
pub struct ParserMessage {
    line_no: u16,
    column: usize,
    message: String,
}

#[maps_to(mruby: mrb_parser_heredoc_info)]
pub struct ParserHeredocInfo<'a> {
    allow_indent: bool,
    line_head: bool,
    string_type: StringType,
    term: &'a str,
    doc: ASTNode<'a>,
}

#[allow(non_camel_case_types)]
pub mod flags {
    pub const STR_PARSING: isize = 0x01;
    pub const STR_EXPAND: isize = 0x02;
    pub const STR_REGEXP: isize = 0x04;
    pub const STR_WORD: isize = 0x08;
    pub const STR_SYMBOL: isize = 0x10;
    pub const STR_ARRAY: isize = 0x20;
    pub const STR_HEREDOC: isize = 0x40;
    pub const STR_XQUOTE: isize = 0x80;
}

#[maps_to(mruby: mrb_string_type)]
/// Represents Ruby's various string types
pub enum StringType {
    NotParsing = 0,
    SQuote = flags::STR_PARSING,
    DQuote = (flags::STR_PARSING | flags::STR_EXPAND),
    Regexp = (flags::STR_PARSING | flags::STR_REGEXP | flags::STR_EXPAND),
    SWord = (flags::STR_PARSING | flags::STR_WORD | flags::STR_ARRAY),
    DWord = (flags::STR_PARSING | flags::STR_WORD | flags::STR_ARRAY | flags::STR_EXPAND),
    SSym = (flags::STR_PARSING | flags::STR_SYMBOL),
    SSymbols = (flags::STR_PARSING | flags::STR_SYMBOL | flags::STR_ARRAY),
    DSymbols = (flags::STR_PARSING | flags::STR_SYMBOL | flags::STR_ARRAY | flags::STR_EXPAND),
    Heredoc = (flags::STR_PARSING | flags::STR_HEREDOC),
    XQuote = (flags::STR_PARSING | flags::STR_XQUOTE | flags::STR_EXPAND),
}
