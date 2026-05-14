use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Json)]
pub fn json_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let fields = match input.data {
        Data::Struct(s) => s
            .fields
            .into_iter()
            .map(|field| field.ident.unwrap().to_string())
            .collect::<Vec<_>>(),
        Data::Enum(e) => e
            .variants
            .into_iter()
            .map(|variant| variant.ident.to_string())
            .collect(),
        Data::Union(u) => u
            .fields
            .named
            .into_iter()
            .map(|field| field.ident.unwrap().to_string())
            .collect(),
    };

    panic!("\n {:#?}", fields);
    quote!().into()
}
