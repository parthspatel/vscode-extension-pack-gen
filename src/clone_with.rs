use serde_json::error::Category::Data;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Ident};

#[proc_macro_derive(CloneWith)]
pub fn clone_with(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let struct_name = &ast.ident;
    let properties = match ast.data {
        Data::Struct(data_struct) => data_struct.fields.into_iter(),
        _ => panic!("WithProperties macro can only be derived for structs."),
    };

    let mut with_methods = Vec::new();
    for field in properties {
        if let Some(field_ident) = field.ident {
            let field_type = field.ty;
            let method_name = Ident::new(&format!("with_{}", field_ident), field_ident.span());

            with_methods.push(quote! {
                pub fn #method_name(mut self, value: #field_type) -> Self {
                    self.#field_ident = value;
                    self
                }
            });
        }
    }

    let output = quote! {
        impl #struct_name {
            #( #with_methods )*
        }
    };

    output.into()
}