use proc_macro::TokenStream;
use syn::{parse::Parser, punctuated::Punctuated, Error, Expr, ExprLit, Lit, Token};

pub fn repeated_impl(input: TokenStream) -> TokenStream {
    let parser = Punctuated::<Expr, Token![,]>::parse_terminated;
    let parsed_input = match parser.parse(input) {
        Ok(p) => p,
        Err(e) => return e.to_compile_error().into(),
    };
    if parsed_input.len() != 2 {
        let msg = "Expected exactly two arguments: a string literal and an integer";

        let span = parsed_input
            .iter()
            .nth(2)
            .map(|_e| proc_macro2::Span::call_site())
            .unwrap_or_else(proc_macro2::Span::call_site);
        return Error::new(span, msg).to_compile_error().into();
    }

    let string_value = match &parsed_input[0] {
        Expr::Lit(ExprLit {
            lit: Lit::Str(s), ..
        }) => s.value(),
        other => {
            return Error::new_spanned(other, "First argument must be a string literal")
                .to_compile_error()
                .into();
        }
    };

    let times_value: usize = match &parsed_input[1] {
        Expr::Lit(ExprLit {
            lit: Lit::Int(i), ..
        }) => match i.base10_parse() {
            Ok(n) => n,
            Err(e) => return e.to_compile_error().into(),
        },
        other => {
            return Error::new_spanned(other, "Second argument must be an integer literal")
                .to_compile_error()
                .into();
        }
    };

    quote::quote! { vec![#string_value.to_string(), #times_value] }.into()
}
