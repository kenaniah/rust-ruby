use crate::VariableTable;

define_object_struct! {
    #[maps_to(mruby: Rclass)]
    RClass {
        instance_variables: &'a VariableTable,
        superclass: &'a Self
    }
}
