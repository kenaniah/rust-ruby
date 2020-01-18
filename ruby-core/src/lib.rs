//! Ruby Core

use std::collections::HashMap;

#[macro_use]
extern crate ruby_proc_macros;

#[macro_use]
mod macros;

mod array;
mod class;
mod compiler;
mod error;
mod hash;
mod memory_pool;
mod object;
mod proc;
mod range;
mod ruby;
mod string;
mod throw;
mod value;
mod variable;

pub use array::{RArray, SharedArray};
pub use class::RClass;
pub use compiler::{ASTNode, CompileContext, LexState, ParserMessage, ParserState, StringType};
pub use error::{RBreak, RException};
pub use hash::RHash;
pub use memory_pool::MemoryPool;
pub use object::{RBasic, RFiber, RObject};
pub use proc::{REnv, RProc};
pub use range::RRange;
pub use ruby::Ruby;
pub use string::{RString, RStringEmbed};
pub use throw::JumpBuffer;
pub use value::{Value, ValueType};
pub use variable::{GlobalEntry, GlobalVariable};

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
