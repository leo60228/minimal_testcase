extern crate proc_macro;

#[proc_macro_attribute]
pub fn dummy(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}
