use darling::util::PathList;
use darling::{FromDeriveInput, FromMeta};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data::Struct, DataStruct, DeriveInput, Field, Fields, Ident, Path};

#[derive(FromMeta, Clone)]
struct CustomModel {
    name: String,
    fields: PathList,

    #[darling(default)]
    extra_derives: PathList,
}

#[derive(FromDeriveInput, Clone)]
#[darling(attributes(custom_model), supports(struct_named))]
struct CustomModelArgs {
    #[darling(multiple, rename = "custom_model")]
    pub models: Vec<CustomModel>,
}

pub(crate) fn derive_custom_model_impl(input: TokenStream) -> TokenStream {
    let orig_struct: DeriveInput = match syn::parse2(input) {
        Ok(val) => val,
        Err(err) => return err.to_compile_error(),
    };

    let DeriveInput { data, .. } = orig_struct.clone();
    if let Struct(data_struct) = data {
        let DataStruct { fields, .. } = data_struct;

        let args = match CustomModelArgs::from_derive_input(&orig_struct) {
            Ok(v) => v,
            Err(e) => return e.write_errors(),
        };

        let CustomModelArgs { models } = args;

        if models.is_empty() {
            return syn::Error::new_spanned(
                &orig_struct.ident,
                "Please specify at least 1 model using the `custom_model` attribute",
            )
            .to_compile_error();
        }

        let mut output = quote!();
        for model in models {
            let generated_model = generate_custom_model(&fields, &model);
            output.extend(generated_model);
        }
        output
    } else {
        syn::Error::new_spanned(
            &orig_struct.ident,
            "DeriveCustomModel can only be used with named structs",
        )
        .to_compile_error()
    }
}

fn generate_custom_model(fields: &Fields, model: &CustomModel) -> TokenStream {
    let CustomModel {
        name,
        fields: target_fields,
        extra_derives,
    } = model;

    let mut new_fields = quote!();

    for Field {
        ident,
        attrs,
        vis,
        colon_token,
        ty,
        ..
    } in fields
    {
        let Some(ident) = ident else { continue };

        let path: Path = ident.clone().into();

        if !target_fields.contains(&path) {
            continue;
        }

        new_fields.extend(quote! {
            #(#attrs)*
            #vis #ident #colon_token #ty,
        });
    }

    let struct_ident = Ident::new(name, proc_macro2::Span::call_site());

    let mut extra_derives_output = quote!();
    if !extra_derives.is_empty() {
        extra_derives_output.extend(quote! {
            #(#extra_derives,)*
        })
    }

    quote! {
        #[derive(#extra_derives_output)]
        pub struct #struct_ident {
            #new_fields
        }
    }
}
