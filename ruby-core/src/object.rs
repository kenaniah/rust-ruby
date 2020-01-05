use crate::VariableTable;

pub(crate) trait Freezable {
    const OBJ_IS_FROZEN: u32 = 1 << 20;
    fn is_frozen(&self) -> bool;
    fn set_frozen(&mut self);
    fn unset_frozen(&mut self);
}

define_object_struct! {
    #[maps_to(mruby: RBasic)]
    RBasic {}
}

define_object_struct! {
    #[maps_to(mruby: RObject)]
    RObject {
        instance_variables: &'a VariableTable
    }
}

define_object_struct! {
    #[maps_to(mruby: RFiber)]
    RFiber {
        //struct mrb_context *cxt;
    }
}
