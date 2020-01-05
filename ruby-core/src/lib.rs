//! Ruby Core

use std::collections::HashMap;

#[macro_use]
extern crate ruby_proc_macros;

#[macro_use]
mod macros;

mod class;
mod compiler;
mod object;
mod ruby_memory_pool;
mod ruby_state;
mod value;

pub use class::RClass;
pub use compiler::{RubyASTNode, RubyCompileContext, RubyParserState};
pub use object::{RBasic, RFiber, RObject};
pub use ruby_memory_pool::RubyMemoryPool;
pub use ruby_state::RubyState;
pub use value::{Value, ValueType};

/// Represents a Ruby `Symbol`
#[maps_to(mruby: mrb_sym)]
pub type Symbol = u32;

/// Represents a table of variables
#[maps_to(mruby: iv_table)]
pub type VariableTable = HashMap<Symbol, Value>;

/// The function pointer type for functions that are callable in Ruby
///
/// The arguments to the function are stored on the `RubyState` struct and can be retrieved via `ruby_state.get_args()`
#[maps_to(mruby: mrb_func_t)]
pub type RubyFunction = fn(ruby_state: &RubyState, sender: Value) -> Value;
