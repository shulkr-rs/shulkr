use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

pub fn expand(input: DeriveInput) -> TokenStream {
    let ident = &input.ident;

    let Data::Enum(data) = &input.data else {
        panic!("PropertyEnum can only be derived for enums");
    };

    let arms = data.variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let string = variant_ident.to_string().to_case(Case::Snake);

        quote! {
            Self::#variant_ident => #string,
        }
    });

    quote! {
        impl PropertyEnum for #ident {
            fn as_str(&self) -> &str {
                match self {
                    #( #arms )*
                }
            }
        }
    }
}
