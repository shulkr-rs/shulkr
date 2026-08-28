use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

pub fn expand(input: DeriveInput) -> TokenStream {
    let ident = &input.ident;

    if !matches!(input.data, Data::Enum(_)) {
        panic!("DataType can only be derived for enums");
    }

    quote! {
        impl crate::protocol::DataType for #ident {
            fn decode<R: crate::protocol::decode::PacketRead>(
                r: &mut R,
            ) -> Result<Self, crate::protocol::decode::DecodeError> {
                Ok(Self::try_from(r.read_varint()?)?)
            }

            fn encode<W: crate::protocol::encode::PacketWrite>(
                w: &mut W,
                this: &Self,
            ) -> Result<(), crate::protocol::encode::EncodeError> {
                w.write_varint(*this as i32)
            }
        }
    }
}
