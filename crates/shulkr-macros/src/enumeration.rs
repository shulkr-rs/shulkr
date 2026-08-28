use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Expr, ExprLit, ExprUnary, Fields, Lit, LitInt, UnOp};

fn discriminant_value(expr: &Expr) -> Option<i64> {
    match expr {
        Expr::Lit(ExprLit {
            lit: Lit::Int(lit_int),
            ..
        }) => lit_int.base10_parse::<i64>().ok(),
        Expr::Unary(ExprUnary {
            op: UnOp::Neg(_),
            expr,
            ..
        }) => discriminant_value(expr).map(|value| -value),
        _ => None,
    }
}

pub fn expand(input: DeriveInput) -> TokenStream {
    let ident = &input.ident;
    let name = ident.to_string();

    let Data::Enum(data) = &input.data else {
        panic!("Enumeration can only be derived for enums");
    };

    let mut next_value: i64 = 0;
    let mut arms = Vec::new();
    let mut variants = Vec::new();
    let mut debug_arms = Vec::new();

    for variant in &data.variants {
        let variant_ident = &variant.ident;

        if !matches!(variant.fields, Fields::Unit) {
            panic!("Enumeration only supports fieldless enum variants");
        }

        let value = match &variant.discriminant {
            Some((_, expr)) => discriminant_value(expr).unwrap_or_else(|| {
                panic!("Enumeration only supports integer literal discriminants")
            }),
            None => next_value,
        };
        next_value = value + 1;

        if value < i32::MIN as i64 || value > i32::MAX as i64 {
            panic!(
                "Enumeration: discriminant {value} of `{ident}::{variant_ident}` does not fit in `i32`"
            );
        }

        let value_lit = LitInt::new(&format!("{value}i32"), Span::call_site());
        arms.push(quote! {
            #value_lit => Ok(Self::#variant_ident),
        });
        variants.push(quote! {
            Self::#variant_ident,
        });

        let variant_name = variant_ident.to_string();
        debug_arms.push(quote! {
            Self::#variant_ident => #variant_name,
        });
    }

    quote! {
        impl #ident {
            pub const fn all() -> &'static [Self] {
                &[ #( #variants )* ]
            }
        }

        impl TryFrom<i32> for #ident {
            type Error = crate::util::InvalidEnumVariant;

            fn try_from(value: i32) -> Result<Self, Self::Error> {
                match value {
                    #( #arms )*
                    _ => Err(crate::util::InvalidEnumVariant::new(#name, value)),
                }
            }
        }

        impl From<#ident> for i32 {
            fn from(value: #ident) -> Self {
                value as i32
            }
        }

        impl ::core::clone::Clone for #ident {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl ::core::marker::Copy for #ident {}

        impl ::core::fmt::Debug for #ident {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(match self {
                    #( #debug_arms )*
                })
            }
        }

        impl ::core::cmp::PartialEq for #ident {
            fn eq(&self, other: &Self) -> bool {
                (*self as i32) == (*other as i32)
            }
        }

        impl ::core::cmp::Eq for #ident {}

        impl ::core::cmp::PartialOrd for #ident {
            fn partial_cmp(&self, other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::option::Option::Some(::core::cmp::Ord::cmp(self, other))
            }
        }

        impl ::core::cmp::Ord for #ident {
            fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                ::core::cmp::Ord::cmp(&(*self as i32), &(*other as i32))
            }
        }

        impl ::core::hash::Hash for #ident {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                ::core::hash::Hash::hash(&(*self as i32), state)
            }
        }
    }
}
