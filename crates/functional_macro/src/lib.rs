mod repeated;
use proc_macro::TokenStream;

#[proc_macro]
pub fn func_proc_macro(input: TokenStream) -> TokenStream {
    repeated::repeated_impl(input)
}
