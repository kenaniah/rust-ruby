#[maps_to(mruby: mrb_shared_array)]
pub struct SharedArray {
    // int refcnt;
    // mrb_int len;
    // mrb_value *ptr;
}

define_object_struct! {
    #[maps_to(mruby: RArray)]
    RArray {
      //   union {
      //   struct {
      //     mrb_int len;
      //     union {
      //       mrb_int capa;
      //       mrb_shared_array *shared;
      //     } aux;
      //     mrb_value *ptr;
      //   } heap;
      //   void *ary[3];
      // } as;
    }
}
