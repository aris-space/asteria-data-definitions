use darling::{FromDeriveInput, FromField, FromVariant, util::Flag};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, spanned::Spanned};

use crate::mavlink::{
    MAVLINK_PRIMITIVES_1BYTE, MAVLINK_PRIMITIVES_2BYTE, MAVLINK_PRIMITIVES_4BYTE,
    MAVLINK_PRIMITIVES_8BYTE,
};

#[derive(FromVariant)]
#[darling(attributes(mavlink))]
pub struct MavlinkDecodeVariant {
    discriminant: Option<syn::Expr>,
    ident: Ident,
}

#[derive(FromField)]
#[darling(attributes(mavlink))]
pub struct MavlinkDecodeField {
    /// Boolean flag whether the field should be flattened
    flatten: Flag,
    /// Boolean flag whether the field should be refined
    refine: Flag,

    ty: syn::Type,
    ident: Option<Ident>,
}

#[derive(FromDeriveInput)]
#[darling(supports(enum_any, struct_any))]
pub struct MavlinkDecode {
    ident: Ident,
    data: darling::ast::Data<MavlinkDecodeVariant, MavlinkDecodeField>,
}

pub fn handle(decode: MavlinkDecode) -> TokenStream {
    if decode.data.is_enum() {
        handle_enum(decode)
    } else {
        handle_struct(decode)
    }
}

fn handle_enum(decode: MavlinkDecode) -> TokenStream {
    let ident = decode.ident;

    let (values, variants) = decode
        .data
        .take_enum()
        .expect("should be enum")
        .into_iter()
        .map(|variant| (variant.discriminant, variant.ident))
        .collect::<(Vec<_>, Vec<_>)>();

    quote! {
        impl ::data_core::mavlink_hal::codec::Decode for #ident {
            #[inline]
            fn decode(
                _: &mut ::data_core::mavlink_hal::codec::tbytes::TBytesReader<
                    impl ::data_core::mavlink_hal::codec::tbytes::TBytesReaderBackend,
                >,
                _: &mut ::data_core::mavlink_hal::codec::tbytes::TBytesReader<
                    impl ::data_core::mavlink_hal::codec::tbytes::TBytesReaderBackend,
                >,
                _: &mut ::data_core::mavlink_hal::codec::tbytes::TBytesReader<
                    impl ::data_core::mavlink_hal::codec::tbytes::TBytesReaderBackend,
                >,
                reader_1_aligned: &mut ::data_core::mavlink_hal::codec::tbytes::TBytesReader<
                    impl ::data_core::mavlink_hal::codec::tbytes::TBytesReaderBackend,
                >,
            ) -> Result<Self, mavlink_hal::codec::tbytes::errors::TBytesError> {
                use ::data_core::mavlink_hal::codec::tbytes::TBytesReaderFor as _;
                reader_1_aligned.read()
            }
        }

        impl<B: ::data_core::mavlink_hal::codec::tbytes::TBytesReaderBackend>
            ::data_core::mavlink_hal::codec::tbytes::TBytesReaderFor<#ident>
            for ::data_core::mavlink_hal::codec::tbytes::TBytesReader<B>
        {
            #[inline]
            fn read(&self) -> Result<#ident, ::data_core::mavlink_hal::codec::tbytes::errors::TBytesError> {
                let variant_id: u8 = self.read()?;
                match variant_id {
                    #(#values => Ok(#ident::#variants)),*,
                    _ => Err(::data_core::mavlink_hal::codec::tbytes::errors::TBytesError::OutOfBounds),
                }
            }
            #[inline]
            fn read_array<const N: usize>(
                &self,
            ) -> Result<[#ident; N], ::data_core::mavlink_hal::codec::tbytes::errors::TBytesError> {
                let mut res = [std::mem::MaybeUninit::uninit(); N];
                for item in &mut res {
                    item.write(self.read()?);
                }

                // SAFETY
                // - We wrote every value of the array in the loop above
                let res: [#ident; N] =
                    unsafe { std::array::from_fn(|i| res[i].assume_init_read()) };

                Ok(res)
            }
        }
    }
}

fn handle_struct(decode: MavlinkDecode) -> TokenStream {
    let ident = decode.ident;

    let (mut bytes1, mut bytes2, mut bytes4, mut bytes8, mut other) =
        (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());

    let mut error = darling::Error::accumulator();
    let allowed_primitives = format!(
        "{}, {}, {}, {}",
        MAVLINK_PRIMITIVES_1BYTE.join(", "),
        MAVLINK_PRIMITIVES_2BYTE.join(", "),
        MAVLINK_PRIMITIVES_4BYTE.join(", "),
        MAVLINK_PRIMITIVES_8BYTE.join(", ")
    );

    decode.data.take_struct().expect("should be struct").fields.into_iter().for_each(|field| {
        if field.flatten.is_present() && field.refine.is_present() {
            error.push(darling::Error::custom("The `#[mavlink(flatten)]` and `#[mavlink(refine)]` attributes cannot be used at the same time!").with_span(&field.ident.span()));
        } else if field.flatten.is_present() || field.refine.is_present() {
            other.push(field.ident);
        } else {
            match &field.ty {
                syn::Type::Path(syn::TypePath { path, .. }) if MAVLINK_PRIMITIVES_1BYTE.iter().any(|p| path.is_ident(p)) => {
                    bytes1.push(field.ident);
                },
                syn::Type::Path(syn::TypePath { path, .. }) if MAVLINK_PRIMITIVES_2BYTE.iter().any(|p| path.is_ident(p)) => {
                    bytes2.push(field.ident);
                },
                syn::Type::Path(syn::TypePath { path, .. }) if MAVLINK_PRIMITIVES_4BYTE.iter().any(|p| path.is_ident(p)) => {
                    bytes4.push(field.ident);
                },
                syn::Type::Path(syn::TypePath { path, .. }) if MAVLINK_PRIMITIVES_8BYTE.iter().any(|p| path.is_ident(p)) => {
                    bytes8.push(field.ident);
                },
                _ => {
                    let error_str = format!("Fields which don't use `#[mavlink(flatten)]` or `#[mavlink(refine)]` have to be of the following primitive types: {allowed_primitives}");
                    error.push(darling::Error::custom(error_str).with_span(&field.ident.span()));
                },
            }
        }
    });

    if let Err(err) = error.finish() {
        return err.write_errors();
    }

    quote! {
        impl ::data_core::mavlink_hal::codec::Decode for #ident {
            #[inline]
            fn decode(
                reader_8_aligned: &mut ::data_core::mavlink_hal::codec::tbytes::TBytesReader<
                    impl ::data_core::mavlink_hal::codec::tbytes::TBytesReaderBackend,
                >,
                reader_4_aligned: &mut ::data_core::mavlink_hal::codec::tbytes::TBytesReader<
                    impl ::data_core::mavlink_hal::codec::tbytes::TBytesReaderBackend,
                >,
                reader_2_aligned: &mut ::data_core::mavlink_hal::codec::tbytes::TBytesReader<
                    impl ::data_core::mavlink_hal::codec::tbytes::TBytesReaderBackend,
                >,
                reader_1_aligned: &mut ::data_core::mavlink_hal::codec::tbytes::TBytesReader<
                    impl ::data_core::mavlink_hal::codec::tbytes::TBytesReaderBackend,
                >,
            ) -> Result<Self, mavlink_hal::codec::tbytes::errors::TBytesError> {
                use ::data_core::mavlink_hal::codec::tbytes::TBytesReaderFor as _;

                Ok(Self {
                    #(#bytes1: reader_1_aligned.read()?,)*
                    #(#bytes2: reader_2_aligned.read()?,)*
                    #(#bytes4: reader_4_aligned.read()?,)*
                    #(#bytes8: reader_8_aligned.read()?,)*
                })
            }
        }
    }
}
