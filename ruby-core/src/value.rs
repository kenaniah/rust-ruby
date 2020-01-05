/// Represents the type of a Ruby `Value`
#[maps_to(mruby: mrb_vtype)]
pub enum ValueType {
    FALSE = 0, /*   0 */
    TRUE,      /*   1 */
    FLOAT,     /*   2 */
    FIXNUM,    /*   3 */
    SYMBOL,    /*   4 */
    UNDEF,     /*   5 */
    CPTR,      /*   6 */
    FREE,      /*   7 */
    OBJECT,    /*   8 */
    CLASS,     /*   9 */
    MODULE,    /*  10 */
    ICLASS,    /*  11 */
    SCLASS,    /*  12 */
    PROC,      /*  13 */
    ARRAY,     /*  14 */
    HASH,      /*  15 */
    STRING,    /*  16 */
    RANGE,     /*  17 */
    EXCEPTION, /*  18 */
    FILE,      /*  19 */
    ENV,       /*  20 */
    DATA,      /*  21 */
    FIBER,     /*  22 */
    ISTRUCT,   /*  23 */
    BREAK,     /*  24 */
}

/// Represents a Ruby `Value`
#[maps_to(mruby: mrb_value)]
pub struct Value {

}
