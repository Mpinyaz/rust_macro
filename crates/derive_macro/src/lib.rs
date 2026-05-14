mod custom_model;
use proc_macro::TokenStream;
use quote::quote;
use syn::DataStruct;
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
}

#[proc_macro_derive(IntoStringHashmap)]
pub fn derive_into_hashmap(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let struct_ident = &input.ident;

    match &input.data {
        Data::Struct(DataStruct { fields, .. }) => {
            let identifiers: Vec<_> = fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();

            quote! {
                #[automatically_derived]
                impl From<#struct_ident> for std::collections::HashMap<String, String> {
                    fn from(value: #struct_ident) -> Self {
                        let mut hash_map = std::collections::HashMap::<String, String>::new();
                        #(
                            hash_map.insert(
                                stringify!(#identifiers).to_string(),
                                String::from(value.#identifiers),
                            );
                        )*
                        hash_map
                    }
                }
            }
        }
        _ => unimplemented!(),
    }
    .into()
}

#[proc_macro_derive(DeriveCustomModel, attributes(custom_model))]
pub fn derive_custom_model(item: TokenStream) -> TokenStream {
    custom_model::derive_custom_model_impl(item.into()).into()
}
