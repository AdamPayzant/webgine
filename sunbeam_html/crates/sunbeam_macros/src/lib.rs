use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(ElementTypeUtils)]
pub fn get_name(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = if let Data::Enum(data_enum) = input.data {
        let variant_arms = data_enum.variants.iter().map(|variant| {
            let variant_name = &variant.ident;

            match &variant.fields {
                Fields::Unit => {
                    quote! {
                        Self::#variant_name => stringify!(#variant_name),
                    }
                }
                Fields::Unnamed(_) => {
                    quote! {
                        Self::#variant_name(..) => stringify!(#variant_name),
                    }
                }
                Fields::Named(_) => {
                    quote! {
                        Self::#variant_name { .. } => stringify!(#variant_name),
                    }
                }
            }
        });

        quote! {
            impl #name {
                pub fn get_name(&self) -> &'static str {
                    match self {
                        #(#variant_arms)*
                    }
                }
            }
        }
    } else {
        panic!("VariantName can only be derived for enums!");
    };

    // Convert the quote tokens into a token stream and return
    TokenStream::from(expanded)
}
