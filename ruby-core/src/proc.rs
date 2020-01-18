use crate::{Symbol, Value};

define_object_struct! {
    #[maps_to(mruby: REnv)]
    REnv {
        stack: &'a Value,
        //struct mrb_context *cxt;
        mid: Symbol
    }
}

define_object_struct! {
    #[maps_to(mruby: RProc)]
    RProc {
      // union {
      //   mrb_irep *irep;
      //   mrb_func_t func;
      // } body;
      // struct RProc *upper;
      // union {
      //   struct RClass *target_class;
      //   struct REnv *env;
      // } e;
    }
}
