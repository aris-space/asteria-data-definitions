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
        impl ::data_core::mavlink::hal::codec::Encode for #ident {
            #[inline]
            fn encode_1_aligned(
                &self,
                writer: &mut ::data_core::mavlink::hal::codec::tbytes::TBytesWriter<
                    impl ::data_core::mavlink::hal::codec::tbytes::TBytesWriterBackend
                >
            ) -> Result<usize, ::data_core::mavlink::hal::codec::tbytes::errors::TBytesError> {
                use ::data_core::mavlink::hal::codec::tbytes::TBytesWriterFor as _;
                writer.write(*self as u8)
            }
        }

        impl<B: ::data_core::mavlink::hal::codec::tbytes::TBytesWriterBackend>
            ::data_core::mavlink::hal::codec::tbytes::TBytesWriterFor<#ident>
            for ::data_core::mavlink::hal::codec::tbytes::TBytesWriter<B>
        where
            #ident: Copy,
        {
            #[inline]
            fn write(&mut self, value: #ident) -> Result<usize, ::data_core::mavlink::hal::codec::tbytes::errors::TBytesError> {
                self.write(value as u8)
            }

            #[inline]
            fn write_slice(&mut self, values: &[#ident]) -> Result<usize, ::data_core::mavlink::hal::codec::tbytes::errors::TBytesError> {
                let mut written = 0usize;
                for value in values {
                    written += self.write(*value)?;
                }
                Ok(written)
            }
        }
    }
}

#[allow(clippy::too_many_lines)]
fn handle_struct(encode: MavlinkEncode) -> TokenStream {
    let ident = encode.ident;

    let (mut bytes1, mut bytes2, mut bytes4, mut bytes8) =
        (Vec::new(), Vec::new(), Vec::new(), Vec::new());

    let mut error = darling::Error::accumulator();
    let allowed_primitives = format!(
        "{}, {}, {}, {}",
        MAVLINK_PRIMITIVES_1BYTE.join(", "),
        MAVLINK_PRIMITIVES_2BYTE.join(", "),
        MAVLINK_PRIMITIVES_4BYTE.join(", "),
        MAVLINK_PRIMITIVES_8BYTE.join(", ")
    );

    encode.data.take_struct().expect("should be struct").fields.into_iter().enumerate().for_each(|(idx, field)| {
        // fields without names are tuple fields, we use a syn::Index to index
        // into them analogous to field names
        let field_ident = if let Some(ident) = field.ident { quote! { #ident } } else {
            let index = syn::Index::from(idx);
            quote! { #index }
        };
        if field.flatten.is_present() && field.refine.is_present() {
            error.push(darling::Error::custom("The `#[mavlink(flatten)]` and `#[mavlink(refine)]` attributes cannot be used at the same time!").with_span(&field_ident.span()));
        } else if field.flatten.is_present() || field.refine.is_present() {
            bytes1.push(quote! { ::data_core::mavlink::hal::codec::Encode::encode_1_aligned(&self.#field_ident, writer) });
            bytes2.push(quote! { ::data_core::mavlink::hal::codec::Encode::encode_2_aligned(&self.#field_ident, writer) });
            bytes4.push(quote! { ::data_core::mavlink::hal::codec::Encode::encode_4_aligned(&self.#field_ident, writer) });
            bytes8.push(quote! { ::data_core::mavlink::hal::codec::Encode::encode_8_aligned(&self.#field_ident, writer) });
        } else {
            let path = match &field.ty {
                syn::Type::Array(syn::TypeArray { elem, .. }) => {
                    if let syn::Type::Path(syn::TypePath { path, .. }) = elem.as_ref() {
                        Some((path, true))
                    } else {
                        None
                    }
                },
                syn::Type::Path(syn::TypePath { path, .. }) => Some((path, false)),
                _ => None
            };

            let mut is_error = path.is_none();
            if let Some((path, array)) = path {
                // bool and [bool; _] types need special treatment as we can only encode u8
                // and [u8; _] respectively but still want to have the convenience of booleans
                let write = match (path.is_ident("bool"), array) {
                    (true, true) => quote! { writer.write_array(self.#field_ident.map(|x| x as u8)) },
                    (true, false) => quote! { writer.write(self.#field_ident as u8) },
                    (false, true) => quote! { writer.write_array(self.#field_ident) },
                    (false, false) => quote! { writer.write(self.#field_ident) },
                };

                match path {
                    _ if MAVLINK_PRIMITIVES_1BYTE.iter().any(|p| path.is_ident(p)) => bytes1.push(write),
                    _ if MAVLINK_PRIMITIVES_2BYTE.iter().any(|p| path.is_ident(p)) => bytes2.push(write),
                    _ if MAVLINK_PRIMITIVES_4BYTE.iter().any(|p| path.is_ident(p)) => bytes4.push(write),
                    _ if MAVLINK_PRIMITIVES_8BYTE.iter().any(|p| path.is_ident(p)) => bytes8.push(write),
                    _ => is_error = true,
                }
            }

            if is_error {
                let error_str = format!("Fields which don't use `#[mavlink(flatten)]` or `#[mavlink(refine)]` have to be of the following primitive types: {allowed_primitives}");
                error.push(darling::Error::custom(error_str).with_span(&field_ident.span()));
            }
        }
    });

    if let Err(err) = error.finish() {
        return err.write_errors();
    }

    quote! {
        impl ::data_core::mavlink::hal::codec::Encode for #ident {
            #[inline]
            fn encode_8_aligned(
                &self,
                writer: &mut ::data_core::mavlink::hal::codec::tbytes::TBytesWriter<
                    impl ::data_core::mavlink::hal::codec::tbytes::TBytesWriterBackend
                >
            ) -> Result<usize, ::data_core::mavlink::hal::codec::tbytes::errors::TBytesError> {
                use ::data_core::mavlink::hal::codec::tbytes::TBytesWriterFor as _;
                let mut written = 0usize;
                #(written += #bytes8?;)*
                Ok(written)
            }

            #[inline]
            fn encode_4_aligned(
                &self,
                writer: &mut ::data_core::mavlink::hal::codec::tbytes::TBytesWriter<
                    impl ::data_core::mavlink::hal::codec::tbytes::TBytesWriterBackend
                >
            ) -> Result<usize, ::data_core::mavlink::hal::codec::tbytes::errors::TBytesError> {
                use ::data_core::mavlink::hal::codec::tbytes::TBytesWriterFor as _;
                let mut written = 0usize;
                #(written += #bytes4?;)*
                Ok(written)
            }

            #[inline]
            fn encode_2_aligned(
                &self,
                writer: &mut ::data_core::mavlink::hal::codec::tbytes::TBytesWriter<
                    impl ::data_core::mavlink::hal::codec::tbytes::TBytesWriterBackend
                >
            ) -> Result<usize, ::data_core::mavlink::hal::codec::tbytes::errors::TBytesError> {
                use ::data_core::mavlink::hal::codec::tbytes::TBytesWriterFor as _;
                let mut written = 0usize;
                #(written += #bytes2?;)*
                Ok(written)
            }

            #[inline]
            fn encode_1_aligned(
                &self,
                writer: &mut ::data_core::mavlink::hal::codec::tbytes::TBytesWriter<
                    impl ::data_core::mavlink::hal::codec::tbytes::TBytesWriterBackend
                >
            ) -> Result<usize, ::data_core::mavlink::hal::codec::tbytes::errors::TBytesError> {
                use ::data_core::mavlink::hal::codec::tbytes::TBytesWriterFor as _;
                let mut written = 0usize;
                #(written += #bytes1?;)*
                Ok(written)
            }
        }
    }
}
