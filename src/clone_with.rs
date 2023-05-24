use serde_json::error::Category::Data;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Ident};

#[proc_macro_derive(CloneWith, attributes(clone_with_method))]
pub fn clone_with(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let struct_name = &ast.ident;
    let properties = match ast.data {
        Data::Struct(data_struct) => data_struct.fields.into_iter(),
        _ => panic!("CloneWith macro can only be derived for structs."),
    };

    let mut clone_methods = Vec::new();
    for field in properties {
        if let Some(field_ident) = field.ident {
            let field_type = field.ty;
            let method_name = Ident::new(&format!("clone_with_{}", field_ident), field_ident.span());

            clone_methods.push(quote! {
                #[clone_with_method]
                pub fn #method_name(&self, value: #field_type) -> Self {
                    let mut cloned = self.clone();
                    cloned.#field_ident = value;
                    cloned
                }
            });
        }
    }

    let output = quote! {
        #[derive(Clone)]
        impl #struct_name {
            #( #clone_methods )*
        }
    };

    output.into()
}

#[derive(CloneWith)]
struct MyStruct {
    #[clone_with_method]
    property1: u32,
    #[clone_with_method]
    property2: String,
    // Add more properties as needed
}

pub fn main() {
    let instance = MyStruct::default()
        .clone_with_property1(42)
        .clone_with_property2("Hello, world!".to_string());
    // You can chain the `clone_with_property_name` calls to set multiple properties

    // Use the updated instance as needed
    println!("{:?}", instance);
}