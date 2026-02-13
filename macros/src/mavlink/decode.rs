use darling::{FromDeriveInput, FromField, FromVariant, util::Flag};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
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
        impl ::data_core::mavlink::hal::codec::Decode for #ident {
            #[inline]
            fn decode(
                _: &mut ::data_core::mavlink::hal::codec::tbytes::TBytesReader<
                    impl ::data_core::mavlink::hal::codec::tbytes::TBytesReaderBackend,
                >,
                _: &mut ::data_core::mavlink::hal::codec::tbytes::TBytesReader<
                    impl ::data_core::mavlink::hal::codec::tbytes::TBytesReaderBackend,
                >,
                _: &mut ::data_core::mavlink::hal::codec::tbytes::TBytesReader<
                    impl ::data_core::mavlink::hal::codec::tbytes::TBytesReaderBackend,
                >,
                reader_1_aligned: &mut ::data_core::mavlink::hal::codec::tbytes::TBytesReader<
                    impl ::data_core::mavlink::hal::codec::tbytes::TBytesReaderBackend,
                >,
            ) -> Result<Self, mavlink::hal::codec::tbytes::errors::TBytesError> {
                use ::data_core::mavlink::hal::codec::tbytes::TBytesReaderFor as _;
                reader_1_aligned.read()
            }
        }

        impl<B: ::data_core::mavlink::hal::codec::tbytes::TBytesReaderBackend>
            ::data_core::mavlink::hal::codec::tbytes::TBytesReaderFor<#ident>
            for ::data_core::mavlink::hal::codec::tbytes::TBytesReader<B>
        {
            #[inline]
            fn read(&self) -> Result<#ident, ::data_core::mavlink::hal::codec::tbytes::errors::TBytesError> {
                let variant_id: u8 = self.read()?;
                match variant_id {
                    #(#values => Ok(#ident::#variants)),*,
                    _ => Err(::data_core::mavlink::hal::codec::tbytes::errors::TBytesError::OutOfBounds),
                }
            }
            #[inline]
            fn read_array<const N: usize>(
                &self,
            ) -> Result<[#ident; N], ::data_core::mavlink::hal::codec::tbytes::errors::TBytesError> {
                let mut res = [core::mem::MaybeUninit::uninit(); N];
                for item in &mut res {
                    item.write(self.read()?);
                }

                // SAFETY
                // - We wrote every value of the array in the loop above
                let res: [#ident; N] =
                    unsafe { core::array::from_fn(|i| res[i].assume_init_read()) };

                Ok(res)
            }
        }
    }
}

fn handle_struct(decode: MavlinkDecode) -> TokenStream {
    let ident = decode.ident;

    let mut reads = Vec::new();

    let mut error = darling::Error::accumulator();
    let allowed_primitives = format!(
        "{}, {}, {}, {}",
        MAVLINK_PRIMITIVES_1BYTE.join(", "),
        MAVLINK_PRIMITIVES_2BYTE.join(", "),
        MAVLINK_PRIMITIVES_4BYTE.join(", "),
        MAVLINK_PRIMITIVES_8BYTE.join(", ")
    );

    let decode_struct = decode.data.take_struct().expect("should be struct");

    decode_struct.fields.iter().for_each(|field| {
        let field_ident = &field.ident;
        if field.flatten.is_present() && field.refine.is_present() {
            error.push(darling::Error::custom("The `#[mavlink(flatten)]` and `#[mavlink(refine)]` attributes cannot be used at the same time!").with_span(&field_ident.span()));
        } else if field.flatten.is_present() || field.refine.is_present() {
            let field_ty = &field.ty;
            reads.push(quote! { #field_ty::decode(reader_8_aligned, reader_4_aligned, reader_2_aligned, reader_1_aligned)? });
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
                let alignment = match path {
                    _ if MAVLINK_PRIMITIVES_1BYTE.iter().any(|p| path.is_ident(p)) => Some(1),
                    _ if MAVLINK_PRIMITIVES_2BYTE.iter().any(|p| path.is_ident(p)) => Some(2),
                    _ if MAVLINK_PRIMITIVES_4BYTE.iter().any(|p| path.is_ident(p)) => Some(4),
                    _ if MAVLINK_PRIMITIVES_8BYTE.iter().any(|p| path.is_ident(p)) => Some(8),
                    _ => {
                        is_error = true;
                        None
                    },
                };

                if let Some(alignment) = alignment {
                    let reader = format_ident!("reader_{alignment}_aligned");
                    // bool and [bool; _] types need special treatment as we can only decode u8
                    // and [u8; _] respectively but still want to have the convenience of booleans
                    let read = match (path.is_ident("bool"), array) {
                        (true, true) => quote! { #reader.read_array()?.map(|x: u8| x != 0) },
                        (true, false) => quote! { #reader.read().map(|x: u8| x != 0)? },
                        (false, true) => quote! { #reader.read_array()? },
                        (false, false) => quote! { #reader.read()? },
                    };
                    reads.push(read);
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

    let instance = if decode_struct.is_tuple() {
        quote! { Self(#(#reads),*) }
    } else {
        let field_idents = decode_struct
            .fields
            .iter()
            .map(|field| &field.ident)
            .collect::<Vec<_>>();
        quote! { Self { #(#field_idents: #reads),* } }
    };

    quote! {
        impl ::data_core::mavlink::hal::codec::Decode for #ident {
            #[inline]
            fn decode(
                reader_8_aligned: &mut ::data_core::mavlink::hal::codec::tbytes::TBytesReader<
                    impl ::data_core::mavlink::hal::codec::tbytes::TBytesReaderBackend,
                >,
                reader_4_aligned: &mut ::data_core::mavlink::hal::codec::tbytes::TBytesReader<
                    impl ::data_core::mavlink::hal::codec::tbytes::TBytesReaderBackend,
                >,
                reader_2_aligned: &mut ::data_core::mavlink::hal::codec::tbytes::TBytesReader<
                    impl ::data_core::mavlink::hal::codec::tbytes::TBytesReaderBackend,
                >,
                reader_1_aligned: &mut ::data_core::mavlink::hal::codec::tbytes::TBytesReader<
                    impl ::data_core::mavlink::hal::codec::tbytes::TBytesReaderBackend,
                >,
            ) -> Result<Self, mavlink::hal::codec::tbytes::errors::TBytesError> {
                use ::data_core::mavlink::hal::codec::tbytes::TBytesReaderFor as _;

                Ok(#instance)
            }
        }
    }
}
