define_object_struct! {
    #[maps_to(mruby: RString)]
    RString {
        //   union {
        //     struct {
        //       mrb_int len;
        //       union {
        //         mrb_int capa;
        //         struct mrb_shared_string *shared;
        //         struct RString *fshared;
        //       } aux;
        //       char *ptr;
        //     } heap;
        //   } as;
    }
}

define_object_struct! {
    #[maps_to(mruby: RStringEmbed)]
    RStringEmbed {
        //   char ary[];
    }
}
