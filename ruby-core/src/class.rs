use crate::{VariableTable};

define_object_struct! {
    #[maps_to(mruby: RClass)]
    RClass {
        instance_variables: &'a VariableTable,
        //struct kh_mt *mt;
        superclass: &'a Self
    }
}
