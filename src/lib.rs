extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(FieldCount)]
pub fn field_count_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let field_count = match &input.data {
        Data::Struct(data) => data.fields.len(),
        _ => panic!("FieldCount can only be derived for structs"),
    };

    let expanded = quote! {
        impl #name {
            pub const fn field_count() -> usize {
                #field_count
            }
        }
    };

    TokenStream::from(expanded)
}
