use crate::{RClass, ValueType};

pub trait Freezable {
    const OBJ_IS_FROZEN: u32 = 1 << 20;
    fn is_frozen(&self) -> bool;
    fn set_frozen(&mut self);
    fn unset_frozen(&mut self);
}

macro_rules! object_struct {
    ($(#[$outer:meta])* $struct:ident {$( $field:ident:$type:ty ),*}) => {
        $(#[$outer])*
        pub struct $struct<'a> {
            c: RClass,
            gc_next: &'a RBasic<'a>,
            tt: ValueType,
            color: u32,
            flags: u32,
            $(
                $field: $type
            )*
        }
        impl Freezable for $struct<'_> {
            fn is_frozen(&self) -> bool {
                self.flags & Self::OBJ_IS_FROZEN > 0
            }
            fn set_frozen(&mut self) {
                self.flags |= Self::OBJ_IS_FROZEN;
            }
            fn unset_frozen(&mut self) {
                self.flags &= !Self::OBJ_IS_FROZEN;
            }
        }
    };
}

object_struct!{
    /// Foobar
    RBasic {}
}

object_struct!{
    /// Blah
    RObject {
        //instance_variables: &'a iv_table
    }
}
object_struct!{
    RFiber {
        //struct mrb_context *cxt;
    }
}
