use crate::{RProc, VariableTable};

define_object_struct! {
    #[maps_to(mruby: RException)]
    RException {
        instance_variables: &'a VariableTable
    }
}

define_object_struct! {
    #[maps_to(mruby: RBreak)]
    RBreak {
        proc: &'a RProc<'a>
        //union mrb_value_union value;
    }
}
