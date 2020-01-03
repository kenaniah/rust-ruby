use crate::{RubyMemoryPool, RubyState, Symbol};

#[cfg(feature = "stdio")]
use std::fs::File;

/**
Implementation of `mrb_parser_state`
*/
pub struct ParserState<'a> {
    /// Ruby's current state (`mrb_state *mrb`)
    pub(crate) ruby_state: &'a RubyState,
    /// Ruby's memory pool (`struct mrb_pool *pool`)
    pub(crate) ruby_pool: &'a RubyMemoryPool<'a>,
    /// (`mrb_ast_node *cells`)
    pub(crate) cells: &'a RubyASTNode<'a>,
    // const char *s, *send;
    pub(crate) s: u8,
    pub(crate) send: u8,
    // FILE *f;
    #[cfg(feature = "stdio")]
    pub(crate) file: &'a File,
    // mrbc_context *cxt;
    // mrb_sym filename_sym;
    // uint16_t lineno;
    // int column;
    //
    // enum mrb_lex_state_enum lstate;
    // mrb_ast_node *lex_strterm; /* (type nest_level beg . end) */
    //
    // unsigned int cond_stack;
    // unsigned int cmdarg_stack;
    // int paren_nest;
    // int lpar_beg;
    // int in_def, in_single;
    // mrb_bool cmd_start:1;
    // mrb_ast_node *locals;
    //
    // mrb_ast_node *pb;
    // char *tokbuf;
    // char buf[MRB_PARSER_TOKBUF_SIZE];
    // int tidx;
    // int tsiz;
    //
    // mrb_ast_node *all_heredocs; /* list of mrb_parser_heredoc_info* */
    // mrb_ast_node *heredocs_from_nextline;
    // mrb_ast_node *parsing_heredoc;
    // mrb_ast_node *lex_strterm_before_heredoc;
    //
    // void *ylval;
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

/**
Used to track file / line information for AST nodes

Corresponds to mruby's `mrb_ast_node` struct
*/
pub struct RubyASTNode<'a> {
    prev: &'a Self,
    next: &'a Self,
    line_no: u16,
    filename_idx: u16
}

/**
Tracks the load context of the parser

Corresponds to mruby's `mrbc_context` struct
*/
pub struct CompileContext<'a> {
    // mrb_sym *syms;
    symbols: &'a Symbol,
    // int slen;
    // char *filename;
    // uint16_t lineno;
    // int (*partial_hook)(struct mrb_parser_state*);
    // void *partial_data;
    // struct RClass *target_class;
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
    parser_err_no: usize
}
