use crate::Value;

define_object_struct! {
    #[maps_to(mruby: RRange)]
    RRange {
        beg: Value,
        end: Value,
        excl: bool
    }
}
