use crate::{Value, VariableTable};
use std::collections::HashMap;

define_object_struct! {
    #[maps_to(mruby: RHash)]
    RHash {
        instance_variables: &'a VariableTable,
        hash_table: &'a HashMap<Value, Value>
    }
}
