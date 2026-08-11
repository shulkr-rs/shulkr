use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, ItemEnum, parse_macro_input};

#[proc_macro_derive(UnitEnum)]
pub fn derive_unit_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let Data::Enum(data) = &input.data else {
        return syn::Error::new_spanned(name, "UnitEnum can only be derived for enums")
            .to_compile_error()
            .into();
    };

    let mut variants = Vec::new();

    for variant in &data.variants {
        match &variant.fields {
            Fields::Unit => variants.push(&variant.ident),
            _ => {
                return syn::Error::new_spanned(
                    &variant.ident,
                    "UnitEnum only supports unit variants",
                )
                .to_compile_error()
                .into();
            }
        }
    }

    quote! {
        impl #name {
            #[inline]
            pub const fn all() -> &'static [Self] {
                &[
                    #(Self::#variants),*
                ]
            }
        }
    }
    .into()
}

#[proc_macro_derive(PropertyEnum)]
pub fn derive_property_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

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
    .into()
}

#[proc_macro_derive(StaticObject)]
pub fn derive_static_object(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemEnum);
    let enum_name = &input.ident;

    // Generate a `Self::Variant => &OBJ_VARIANT` match arm for each variant
    let arms = input.variants.iter().map(|v| {
        let variant_ident = &v.ident;

        // Converts "Air" -> "OBJ_AIR", "Stone" -> "OBJ_STONE"
        let static_name = format!("{}", variant_ident.to_string().to_case(Case::Constant));
        let static_ident = syn::Ident::new(&static_name, variant_ident.span());

        quote! {
            Self::#variant_ident => &#static_ident,
        }
    });

    let name = format_ident!("{}Data", enum_name);
    
    let expanded = quote! {
        impl #enum_name {
            /// Matches the enum variant to its corresponding static object
            pub const fn data(&self) -> &'static #name {
                match self {
                    #(#arms)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

use syn::{ImplItem, ImplItemConst, ItemImpl};

#[proc_macro_attribute]
pub fn registry_values(_attr: TokenStream, item: TokenStream) -> TokenStream {
    const MOD_NAME: &str = "static_objects";
    let mod_ident = format_ident!("{}", MOD_NAME);

    let input = parse_macro_input!(item as ItemImpl);
    let self_ty = &input.self_ty; // `Test`

    let mut statics = Vec::new();
    let mut impl_consts = Vec::new();

    for item in &input.items {
        if let ImplItem::Const(ImplItemConst { ident, expr, .. }) = item {
            // pub static TEST1: Holder<Test> = Holder::new("key");
            statics.push(quote! {
                pub static #ident: Holder<super::#self_ty> = #expr;
            });
            // pub const TEST1: &Holder<Test> = &static_objects::TEST1;
            impl_consts.push(quote! {
                pub const #ident: &'static Holder<#self_ty> = &#mod_ident::#ident;
            });
        }
    }

    let expanded = quote! {
        mod #mod_ident {
            use super::*;

            #(#statics)*
        }

        impl #self_ty {
            #(#impl_consts)*
        }
    };

    expanded.into()
}
