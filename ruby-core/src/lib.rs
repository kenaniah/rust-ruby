//! Ruby Core

use std::collections::HashMap;

#[macro_use]
mod macros;

mod class;
mod compiler;
mod object;
mod ruby_memory_pool;
mod ruby_state;
mod value;

pub use class::RClass;
pub use compiler::{CompileContext, ParserState, RubyASTNode};
pub use object::{RBasic, RObject, RFiber};
pub use ruby_state::RubyState;
pub use ruby_memory_pool::RubyMemoryPool;
pub use value::{Value, ValueType};

/**
Represents a Ruby `Symbol`
*/
pub type Symbol = u32;

/**
Represents a table of variables

Corresponds to mruby's `iv_table` struct
*/
pub type VariableTable = HashMap<Symbol, Value>;
