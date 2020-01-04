use crate::VariableTable;

define_object_struct!{
    RClass {
        instance_variables: &'a VariableTable,
        superclass: &'a Self
    }
}
