use darling::{FromDeriveInput, FromVariant};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use stringcase::snake_case;
use syn::{Ident, LitStr};

use crate::util::GetDocs;

#[derive(FromVariant)]
#[darling(attributes(mavlink), forward_attrs(doc))]
pub struct MavlinkEnumVariant {
    /// Optional name override for the variant. If left empty, the upper snake
    /// case variant name will be used.
    ///
    /// **Note**: The name override will be converted to a upper snake case string
    name: Option<String>,

    ident: Ident,
    discriminant: Option<syn::Expr>,
    attrs: Vec<syn::Attribute>,
}

#[derive(FromDeriveInput)]
#[darling(supports(enum_unit), attributes(mavlink), forward_attrs(doc))]
pub struct MavlinkEnum {
    /// Optional name override for the enum. If left empty, the upper snake
    /// case enum name will be used.
    ///
    /// **Note**: The name override will be converted to a upper snake case string
    name: Option<String>,

    ident: Ident,
    data: darling::ast::Data<MavlinkEnumVariant, ()>,
    attrs: Vec<syn::Attribute>,
}

pub fn handle(mavlink_enum: MavlinkEnum) -> TokenStream {
    let ident = mavlink_enum.ident;
    let description_lit = LitStr::new(&mavlink_enum.attrs.get_docs_str(), Span::call_site());
    let name = snake_case(&mavlink_enum.name.clone().unwrap_or(ident.to_string())).to_uppercase();
    let name_lit = LitStr::new(&name, Span::call_site());

    let variants = mavlink_enum
        .data
        .take_enum()
        .unwrap_or_default()
        .into_iter()
        .map(|variant| {
            let description_lit = LitStr::new(&variant.attrs.get_docs_str(), Span::call_site());
            let name =
                snake_case(&variant.name.unwrap_or(variant.ident.to_string())).to_uppercase();
            let name_lit = LitStr::new(&name, Span::call_site());
            let discriminant = variant.discriminant;

            quote! {
                ::data_core::mavlink_hal::description::EnumVariant {
                    value: #discriminant,
                    name: #name_lit,
                    description: #description_lit
                }
            }
        })
        .collect::<Vec<_>>();

    quote! {
        impl ::data_core::mavlink_hal::description::DescribeEnum for #ident {
            const ENUM: &'static ::data_core::mavlink_hal::description::Enum = &::data_core::mavlink_hal::description::Enum {
                name: #name_lit,
                description: #description_lit,
                variants: &[#(#variants),*]
            };
        }

        impl ::data_core::mavlink_hal::typing::ScalarTypeable for #ident {
            const SCALAR_TYPE: ::data_core::mavlink_hal::typing::MavScalarType = <u8 as ::data_core::mavlink_hal::typing::ScalarTypeable>::SCALAR_TYPE;
            const SCALAR_ENUM: Option<&'static ::data_core::mavlink_hal::description::Enum> = Some(<#ident as ::data_core::mavlink_hal::description::DescribeEnum>::ENUM);
        }
    }
}
