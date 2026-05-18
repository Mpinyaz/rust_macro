mod struct_modifier;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn func_struct_expand(attrs: TokenStream, input: TokenStream) -> TokenStream {
    struct_modifier::struct_expand_impl(attrs, input)
}
