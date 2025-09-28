use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

#[proc_macro_derive(Random)]
pub fn random_derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    // get enum name
    let name = &input.ident;
    let data = &input.data;

    // Generate code for the random function
    let r#gen = match data {
        Data::Enum(data_enum) => {
            let variants: Vec<_> = data_enum
                .variants
                .iter()
                .map(|v| {
                    let ident = &v.ident;
                    quote! { #name::#ident }
                })
                .collect();

            quote! {
                impl #name {
                    pub fn random() -> Self {
                        use rand::seq::SliceRandom;
                        let mut rng = rand::rng();
                        let variants: &[#name] = &[ #(#variants),* ];
                        *variants.choose(&mut rng).unwrap()
                    }
                }
            }
        }
        _ => panic!("Random can only be derived for enums"),
    };

    r#gen.into()
}
