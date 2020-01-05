//! Ruby Core

use std::collections::HashMap;

#[macro_use]
extern crate ruby_proc_macros;

#[macro_use]
mod macros;

mod array;
mod class;
mod compiler;
mod object;
mod memory_pool;
mod ruby;
mod value;

pub use array::{RArray, SharedArray};
pub use class::RClass;
pub use compiler::{ASTNode, CompileContext, LexState, ParserMessage, ParserState, StringType};
pub use object::{RBasic, RFiber, RObject};
pub use memory_pool::MemoryPool;
pub use ruby::Ruby;
pub use value::{Value, ValueType};

/// Represents a Ruby `Symbol`
#[maps_to(mruby: mrb_sym)]
pub type Symbol = u32;

/// Represents a table of variables
#[maps_to(mruby: iv_table)]
pub type VariableTable = HashMap<Symbol, Value>;

/// Function pointer type for functions that are callable in Ruby
///
/// The arguments to the function are stored on the `State` struct and can be retrieved via `ruby_state.get_args()`
#[maps_to(mruby: mrb_func_t)]
pub type RubyFunction = fn(rb: &Ruby, sender: Value) -> Value;
