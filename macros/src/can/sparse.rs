use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, PathSegment, Token, parse::Parse, punctuated::Punctuated, spanned::Spanned};

use crate::{can::message::CAN_MESSAGE_META_CAN_IDS_IDENT, message::MESSAGE_META_TYPES_IDENT};

struct SparseCanMessageVariant {
    /// Path segment representing the module where the original message enum
    /// exists
    module_segment: PathSegment,
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
        if path.len() != 3 {
            Err(syn::Error::new(
                path.span(),
                "Expected `message_module::message_enum::variant_name` where `message_module` is the module where the CAN message `message_enum` is defined and `variant_name` is the name of the variant within this message enum",
            ))?;
        }

        let module_segment = path.first().expect("this is already checked above").clone();
        let variant_segment = path.last().expect("this is already checked above").clone();

        if input.peek(Token![,]) {
            let _ = input.parse::<Token![,]>();
        }

        Ok(Self {
            module_segment,
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
                let mut type_segments = Punctuated::new();
                type_segments.push(variant.module_segment.clone());
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

                let mut id_segments = Punctuated::new();
                id_segments.push(variant.module_segment.clone());
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
