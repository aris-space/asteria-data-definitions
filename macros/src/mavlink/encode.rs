use darling::{FromDeriveInput, FromField, util::Flag};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, spanned::Spanned};

use crate::mavlink::{
    MAVLINK_PRIMITIVES_1BYTE, MAVLINK_PRIMITIVES_2BYTE, MAVLINK_PRIMITIVES_4BYTE,
    MAVLINK_PRIMITIVES_8BYTE,
};

#[derive(FromField)]
#[darling(attributes(mavlink))]
pub struct MavlinkEncodeField {
    /// Boolean flag whether the field should be flattened
    flatten: Flag,
    /// Boolean flag whether the field should be refined
    refine: Flag,

    ty: syn::Type,
    ident: Option<Ident>,
}

#[derive(FromDeriveInput)]
#[darling(supports(enum_any, struct_any))]
pub struct MavlinkEncode {
    ident: Ident,
    data: darling::ast::Data<syn::Variant, MavlinkEncodeField>,
}

pub fn handle(encode: MavlinkEncode) -> TokenStream {
    if encode.data.is_enum() {
        handle_enum(encode)
    } else {
        handle_struct(encode)
    }
}

fn handle_enum(encode: MavlinkEncode) -> TokenStream {
    let ident = encode.ident;
    quote! {
        impl ::data_core::mavlink_hal::codec::Encode for #ident {
            #[inline]
            fn encode_1_aligned(
                &self,
                writer: &mut ::data_core::mavlink_hal::codec::tbytes::TBytesWriter<
                    impl ::data_core::mavlink_hal::codec::tbytes::TBytesWriterBackend
                >
            ) -> Result<usize, ::data_core::mavlink_hal::codec::tbytes::errors::TBytesError> {
                use ::data_core::mavlink_hal::codec::tbytes::TBytesWriterFor as _;
                writer.write(*self as u8)
            }
        }

        impl<B: ::data_core::mavlink_hal::codec::tbytes::TBytesWriterBackend>
            ::data_core::mavlink_hal::codec::tbytes::TBytesWriterFor<#ident>
            for ::data_core::mavlink_hal::codec::tbytes::TBytesWriter<B>
        where
            #ident: Copy,
        {
            #[inline]
            fn write(&mut self, value: #ident) -> Result<usize, ::data_core::mavlink_hal::codec::tbytes::errors::TBytesError> {
                self.write(value as u8)
            }

            #[inline]
            fn write_slice(&mut self, values: &[#ident]) -> Result<usize, ::data_core::mavlink_hal::codec::tbytes::errors::TBytesError> {
                let mut written = 0usize;
                for value in values {
                    written += self.write(*value)?;
                }
                Ok(written)
            }
        }
    }
}

fn handle_struct(encode: MavlinkEncode) -> TokenStream {
    let ident = encode.ident;

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

    encode.data.take_struct().expect("should be struct").fields.into_iter().for_each(|field| {
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
        impl ::data_core::mavlink_hal::codec::Encode for #ident {
            #[inline]
            fn encode_8_aligned(
                &self,
                writer: &mut ::data_core::mavlink_hal::codec::tbytes::TBytesWriter<
                    impl ::data_core::mavlink_hal::codec::tbytes::TBytesWriterBackend
                >
            ) -> Result<usize, ::data_core::mavlink_hal::codec::tbytes::errors::TBytesError> {
                use ::data_core::mavlink_hal::codec::tbytes::TBytesWriterFor as _;
                let mut written = 0usize;

                #(written += writer.write(self.#bytes8)?;)*
                #(written += self.#other.encode_8_aligned(writer)?;)*

                Ok(written)
            }

            #[inline]
            fn encode_4_aligned(
                &self,
                writer: &mut ::data_core::mavlink_hal::codec::tbytes::TBytesWriter<
                    impl ::data_core::mavlink_hal::codec::tbytes::TBytesWriterBackend
                >
            ) -> Result<usize, ::data_core::mavlink_hal::codec::tbytes::errors::TBytesError> {
                use ::data_core::mavlink_hal::codec::tbytes::TBytesWriterFor as _;
                let mut written = 0usize;

                #(written += writer.write(self.#bytes4)?;)*
                #(written += self.#other.encode_4_aligned(writer)?;)*

                Ok(written)
            }

            #[inline]
            fn encode_2_aligned(
                &self,
                writer: &mut ::data_core::mavlink_hal::codec::tbytes::TBytesWriter<
                    impl ::data_core::mavlink_hal::codec::tbytes::TBytesWriterBackend
                >
            ) -> Result<usize, ::data_core::mavlink_hal::codec::tbytes::errors::TBytesError> {
                use ::data_core::mavlink_hal::codec::tbytes::TBytesWriterFor as _;
                let mut written = 0usize;

                #(written += writer.write(self.#bytes2)?;)*
                #(written += self.#other.encode_2_aligned(writer)?;)*

                Ok(written)
            }

            #[inline]
            fn encode_1_aligned(
                &self,
                writer: &mut ::data_core::mavlink_hal::codec::tbytes::TBytesWriter<
                    impl ::data_core::mavlink_hal::codec::tbytes::TBytesWriterBackend
                >
            ) -> Result<usize, ::data_core::mavlink_hal::codec::tbytes::errors::TBytesError> {
                use ::data_core::mavlink_hal::codec::tbytes::TBytesWriterFor as _;
                let mut written = 0usize;

                #(written += writer.write(self.#bytes1)?;)*
                #(written += self.#other.encode_1_aligned(writer)?;)*

                Ok(written)
            }
        }
    }
}
