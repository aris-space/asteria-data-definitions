use darling::{FromDeriveInput, FromVariant, ast::Fields};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::Ident;

#[derive(FromVariant)]
#[darling(attributes(can))]
pub struct CanMessageVariant {
    /// Unique id of the CAN message variant
    id: u16,

    ident: Ident,
    fields: Fields<syn::Field>,
}

#[derive(FromDeriveInput)]
#[darling(supports(enum_any))]
pub struct CanMessage {
    pub ident: Ident,
    pub data: darling::ast::Data<CanMessageVariant, ()>,
}

pub fn handle(can_message: CanMessage) -> TokenStream {
    let variants = can_message.data.take_enum().unwrap_or_default();
    // TODO: Ensure every can-id is only used once
    //       Should we do that?

    let meta_types_mod = format_ident!("__message_meta_types");
    let meta_types = meta_types(&variants);

    let meta_can_ids_mod = format_ident!("__message_meta_can_ids");
    let meta_can_ids = meta_can_ids(&variants);

    let can_decode = impl_can_decode(
        &can_message.ident,
        &variants,
        &meta_can_ids_mod,
        &meta_types_mod,
    );
    let can_encode = impl_can_encode(
        &can_message.ident,
        &variants,
        &meta_can_ids_mod,
        &meta_types_mod,
    );

    quote! {
        pub mod #meta_types_mod {
            #[allow(clippy::wildcard_imports)]
            use super::*;

            #meta_types
        }

        #[allow(non_upper_case_globals)]
        pub mod #meta_can_ids_mod {
            #meta_can_ids
        }

        #can_decode
        #can_encode
    }
}

fn meta_types(variants: &[CanMessageVariant]) -> TokenStream {
    let type_defs = variants
        .iter()
        .map(|variant| {
            let (types, type_idents) = variant
                .fields
                .iter()
                .map(|field| (&field.ty, &field.ident))
                .collect::<(Vec<_>, Vec<_>)>();

            let ident = &variant.ident;

            if variant.fields.is_struct() {
                quote! {
                    #[derive(bincode::Encode, bincode::Decode)]
                    pub struct #ident {
                        #(pub #type_idents: #types),*
                    }
                }
            } else if variant.fields.is_unit() {
                quote! { pub type #ident = (); }
            } else if variant.fields.is_newtype()
                && let Some(field_type) = types.first()
            {
                quote! { pub type #ident = #field_type; }
            } else {
                quote! { pub type #ident = (#(#types),*); }
            }
        })
        .collect::<Vec<_>>();

    quote! {
        #(#type_defs)*
    }
}

fn meta_can_ids(variants: &[CanMessageVariant]) -> TokenStream {
    let (idents, ids) = variants
        .iter()
        .map(|variant| (&variant.ident, variant.id))
        .collect::<(Vec<_>, Vec<_>)>();

    let length = idents.len();

    quote! {
        #(pub const #idents: can_hal::StandardId = can_hal::StandardId::new(#ids).expect("must be in range");)*

        pub const ALL_IDS: [can_hal::StandardId; #length] = [
            #(#idents),*
        ];
    }
}

fn impl_can_decode(
    ident: &Ident,
    variants: &[CanMessageVariant],
    ids_mod_ident: &Ident,
    types_mod_ident: &Ident,
) -> TokenStream {
    let (idents, mappings) = variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.ident;

            let variant_mapping = if variant.fields.is_unit() {
                quote! { Ok(Self::#variant_ident) }
            } else if variant.fields.is_struct() {
                let type_idents = variant
                    .fields
                    .iter()
                    .map(|field| &field.ident)
                    .collect::<Vec<_>>();

                quote! {
                    can_hal::codec::decode(data)
                        .map(|d: #types_mod_ident::#variant_ident| Self::#variant_ident { #(#type_idents: d.#type_idents),* })
                }
            } else if variant.fields.is_newtype() {
                quote! { can_hal::codec::decode(data).map(Self::#variant_ident) }
            } else {
                let indices = (0..variant.fields.len()).map(syn::Index::from).collect::<Vec<_>>();
                quote! {
                    can_hal::codec::decode(data)
                        .map(|d: #types_mod_ident::#variant_ident| Self::#variant_ident (#(d.#indices),*))
                }
            };

            (variant_ident, variant_mapping)
        })
        .collect::<(Vec<_>, Vec<_>)>();

    quote! {
        impl can_hal::CanDecode for #ident {
            type Error = can_hal::codec::Error;

            fn from_parts(id: can_hal::StandardId, data: &[u8]) -> Result<Self, Self::Error> {
                match id {
                    #(#ids_mod_ident::#idents => #mappings,)*
                    _ => Err(Self::Error::UnknownId),
                }
            }
        }
    }
}

fn impl_can_encode(
    ident: &Ident,
    variants: &[CanMessageVariant],
    ids_mod_ident: &Ident,
    types_mod_ident: &Ident,
) -> TokenStream {
    let (idents, mappings) = variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.ident;

            if variant.fields.is_unit() {
                let mapping = quote! { Ok((#ids_mod_ident::#variant_ident, 0)) };
                (quote!(#variant_ident), mapping)
            } else if variant.fields.is_struct() {
                let type_idents = variant
                    .fields
                    .iter()
                    .map(|field| &field.ident)
                    .collect::<Vec<_>>();

                let ident = quote! { #variant_ident { #(#type_idents),* } };
                let mapping = quote! {
                    Ok((
                        #ids_mod_ident::#variant_ident,
                        can_hal::codec::encode(&#types_mod_ident::#variant_ident { #(#type_idents: *#type_idents),* }, buf)?
                    ))
                };
                (ident, mapping)
            } else if variant.fields.is_newtype() {
                let ident = quote! {#variant_ident(x)};
                let mapping =
                    quote! { Ok((#ids_mod_ident::#variant_ident, can_hal::codec::encode(x, buf)?)) };
                (ident, mapping)
            } else {
                let type_idents = (0..variant.fields.len()).map(|i| format_ident!("field{i}")).collect::<Vec<_>>();

                let ident = quote! { #variant_ident(#(#type_idents),*) };
                let mapping = quote! {
                    Ok((
                        #ids_mod_ident::#variant_ident,
                        can_hal::codec::encode(&(#(*#type_idents),*), buf)?
                    ))
                };
                (ident, mapping)
            }
        })
        .collect::<(Vec<_>, Vec<_>)>();

    quote! {
        impl can_hal::CanEncode for #ident {
            type Error = can_hal::codec::Error;

            fn encode_into(&self, buf: &mut [u8; 64]) -> Result<(can_hal::StandardId, u8), Self::Error> {
                match self {
                    #(Self::#idents => #mappings,)*
                }
            }
        }
    }
}
