use darling::{util::PathList, FromMeta, ast::NestedMeta};
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemStruct, Fields, parse_quote};
use quote::{quote, format_ident};

#[derive(Clone, Debug)]
struct NewFieldDef {
    name: syn::Ident,
    ty: syn::Type,
}

impl FromMeta for NewFieldDef {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        if items.len() != 2 {
            return Err(darling::Error::custom("Expected two arguments: name and type, e.g., fields(nam, \"u32\")"));
        }

        let name = match &items[0] {
            NestedMeta::Meta(syn::Meta::Path(path)) => path.get_ident()
                .cloned()
                .ok_or_else(|| darling::Error::custom("First argument must be an identifier")),
            _ => Err(darling::Error::custom("First argument must be an identifier")),
        }?;

        let ty = match &items[1] {
            NestedMeta::Lit(syn::Lit::Str(s)) => s.parse::<syn::Type>()
                .map_err(|_| darling::Error::custom("Second argument must be a valid type string")),
            NestedMeta::Meta(syn::Meta::Path(path)) => Ok(parse_quote!(#path)),
            _ => Err(darling::Error::custom("Second argument must be a type (path or string literal)")),
        }?;

        Ok(NewFieldDef { name, ty })
    }
}

#[derive(FromMeta, Clone, Debug)]
struct AddFields {
    #[darling(multiple, rename = "fields")]
    add_fields: Vec<NewFieldDef>,
    #[darling(default)]
    setter: PathList,
    #[darling(default)]
    getter: PathList,
}

pub fn struct_expand_impl(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(attrs.into()) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(darling::Error::from(e).write_errors()),
    };

    let args = match AddFields::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let mut input_struct = parse_macro_input!(input as ItemStruct);

    // Add new fields to the struct
    if let Fields::Named(ref mut fields) = input_struct.fields {
        for field_def in args.add_fields {
            let name = field_def.name;
            let ty = field_def.ty;
            fields.named.push(parse_quote! {
                pub #name: #ty
            });
        }
    } else {
        return syn::Error::new_spanned(&input_struct, "Only structs with named fields are supported")
            .to_compile_error()
            .into();
    }

    let struct_name = &input_struct.ident;
    let (impl_generics, ty_generics, where_clause) = input_struct.generics.split_for_impl();

    let mut methods = Vec::new();

    // Helper to find a field by name (searching both original and newly added fields)
    let get_field = |ident: &syn::Ident| {
        if let Fields::Named(fields) = &input_struct.fields {
            fields.named.iter().find(|f| f.ident.as_ref() == Some(ident))
        } else {
            None
        }
    };

    // Generate getters
    for path in args.getter.iter() {
        if let Some(ident) = path.get_ident() {
            if let Some(field) = get_field(ident) {
                let ty = &field.ty;
                methods.push(quote! {
                    pub fn #ident(&self) -> &#ty {
                        &self.#ident
                    }
                });
            } else {
                return syn::Error::new_spanned(path, format!("Field `{}` not found", ident))
                    .to_compile_error()
                    .into();
            }
        }
    }

    // Generate setters
    for path in args.setter.iter() {
        if let Some(ident) = path.get_ident() {
            if let Some(field) = get_field(ident) {
                let ty = &field.ty;
                let setter_name = format_ident!("set_{}", ident);
                methods.push(quote! {
                    pub fn #setter_name(&mut self, val: #ty) {
                        self.#ident = val;
                    }
                });
            } else {
                return syn::Error::new_spanned(path, format!("Field `{}` not found", ident))
                    .to_compile_error()
                    .into();
            }
        }
    }

    let expanded = quote! {
        #input_struct

        impl #impl_generics #struct_name #ty_generics #where_clause {
            #(#methods)*
        }
    };

    TokenStream::from(expanded)
}
