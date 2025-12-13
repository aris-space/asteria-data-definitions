use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, PathSegment, Token, parse::Parse, punctuated::Punctuated, spanned::Spanned};

use crate::{can::message::CAN_MESSAGE_META_CAN_IDS_IDENT, message::MESSAGE_META_TYPES_IDENT};

struct SparseCanMessageVariant {
    /// Path segments representing the module path where the original message enum exists
    module_segments: Punctuated<PathSegment, Token![::]>,
    /// Path segment representing the name of a variant of the original message enum
    variant_segment: PathSegment,
    /// Identifier of the enum which should be newly created
    ident: Ident,
}

impl Parse for SparseCanMessageVariant {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let parser = Punctuated::<PathSegment, Token![::]>::parse_terminated;

        let ident = input.parse::<Ident>()?;

        let content;
        syn::parenthesized!(content in input);

        let path = parser(&content)?;
        let len = path.len();
        if len < 2 {
            Err(syn::Error::new(
                path.span(),
                "Expected `message_enum::variant_name` where `message_enum` is the CAN message enum and `variant_name` is the name of the variant within this message enum",
            ))?;
        }

        let variant_segment = path.last().expect("this is already checked above").clone();
        let module_segments = path.into_iter().take(len - 2).collect();

        if input.peek(Token![,]) {
            let _ = input.parse::<Token![,]>();
        }

        Ok(Self {
            module_segments,
            variant_segment,
            ident,
        })
    }
}

pub struct SparseCanMessage {
    ident: Ident,
    variants: Vec<SparseCanMessageVariant>,
}

impl Parse for SparseCanMessage {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Token![enum]>()?;
        let ident = input.parse::<Ident>()?;

        let content;
        syn::braced!(content in input);

        let mut variants = Vec::new();
        while !content.is_empty() {
            variants.push(content.parse()?);
        }

        Ok(Self { ident, variants })
    }
}

impl SparseCanMessage {
    fn as_enum(&self) -> TokenStream {
        let ident = &self.ident;
        let (type_paths, id_paths, idents) = self
            .variants
            .iter()
            .map(|variant| {
                let mut type_segments = variant.module_segments.clone();
                type_segments.push_punct(syn::token::PathSep::default());
                type_segments.push(PathSegment::from(Ident::new(
                    MESSAGE_META_TYPES_IDENT,
                    Span::call_site(),
                )));
                type_segments.push_punct(syn::token::PathSep::default());
                type_segments.push(variant.variant_segment.clone());

                let type_path = syn::Path {
                    leading_colon: None,
                    segments: type_segments,
                };

                let mut id_segments = variant.module_segments.clone();
                id_segments.push_punct(syn::token::PathSep::default());
                id_segments.push(PathSegment::from(Ident::new(
                    CAN_MESSAGE_META_CAN_IDS_IDENT,
                    Span::call_site(),
                )));
                id_segments.push_punct(syn::token::PathSep::default());
                id_segments.push(variant.variant_segment.clone());

                let id_path = syn::Path {
                    leading_colon: None,
                    segments: id_segments,
                };

                (type_path, id_path, &variant.ident)
            })
            .collect::<(Vec<_>, Vec<_>, Vec<_>)>();

        quote! {
            pub enum #ident {
                #(
                    #[can(
                        variant_id = #id_paths
                    )]
                    #idents(#type_paths),
                )*
            }
        }
    }
}

pub fn handle_encode(message: &SparseCanMessage) -> TokenStream {
    let enum_stream = message.as_enum();
    quote! {
        #[derive(::data_core::CanEncode)]
        #[can(sparse)]
        #enum_stream
    }
}

pub fn handle_decode(message: &SparseCanMessage) -> TokenStream {
    let enum_stream = message.as_enum();
    quote! {
        #[derive(::data_core::CanDecode)]
        #[can(sparse)]
        #enum_stream
    }
}
