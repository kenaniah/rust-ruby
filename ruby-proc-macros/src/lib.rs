extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn maps_to(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // This macro should eventually add a section to the item's documentation that displays
    // references to other Ruby implementations
    item
}
