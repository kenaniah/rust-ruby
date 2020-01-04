/// Used to define the Ruby object structs
#[macro_export]
macro_rules! define_object_struct {
    ($(#[$outer:meta])* $struct:ident {$( $field:ident:$type:ty ),*}) => {
        $(#[$outer])*
        pub struct $struct<'a> {
            c: &'a crate::RClass<'a>,
            gc_next: &'a crate::RBasic<'a>,
            tt: crate::ValueType,
            color: u32,
            flags: u32,
            $(
                $field: $type,
            )*
        }
        impl crate::object::Freezable for $struct<'_> {
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
