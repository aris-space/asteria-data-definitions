use darling::{FromDeriveInput, FromVariant, ast::Fields};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, LitInt, LitStr};

#[derive(FromVariant)]
pub struct MavlinkDialectVariant {
    fields: Fields<syn::Field>,
    ident: Ident,
}

#[derive(FromDeriveInput)]
#[darling(supports(enum_newtype), attributes(dialect))]
pub struct MavlinkDialect {
    /// Name of the generated MavLink dialect
    name: LitStr,
    /// Numeric identifier of the generated MavLink dialect
    id: LitInt,
    /// Version of the generated MavLink dialect
    version: LitInt,

    ident: Ident,
    data: darling::ast::Data<MavlinkDialectVariant, ()>,
}

pub fn handle(mavlink_dialect: MavlinkDialect) -> TokenStream {
    let MavlinkDialect {
        name,
        id,
        version,
        ident,
        data,
    } = mavlink_dialect;

    let variants = data.take_enum().unwrap_or_default();
    let describe_messages_impl = generate_describe_messages_impl(&ident, &variants);
    let message_impl = generate_message_impl(&ident, &variants);

    let (idents, types) = variants
        .iter()
        .map(|variant| {
            let field = variant
                .fields
                .fields
                .first()
                .expect("should have exactly one field since it's a newtype");

            (&variant.ident, &field.ty)
        })
        .collect::<(Vec<_>, Vec<_>)>();

    quote! {
        #describe_messages_impl
        #message_impl

        impl ::data_core::mavlink::mavio::Dialect for #ident {
            fn name() -> &'static str {
                #name
            }

            fn dialect() -> Option<::data_core::mavlink::mavio::protocol::DialectId> {
                core::option::Option::Some(#id)
            }

            fn version() -> Option<::data_core::mavlink::mavio::protocol::DialectVersion> {
                core::option::Option::Some(#version)
            }

            fn message_info(
                id: ::data_core::mavlink::mavio::protocol::MessageId,
            ) -> Result<&'static dyn ::data_core::mavlink::mavio::protocol::MessageSpec, ::data_core::mavlink::mavio::prelude::SpecError> {
                <Self as ::data_core::mavlink::hal::description::DescribeMessages>::MESSAGES
                    .iter()
                    .find(|m| m.id == id)
                    .map(|m| m as &'static dyn ::data_core::mavlink::mavio::protocol::MessageSpec)
                    .ok_or(::data_core::mavlink::mavio::prelude::SpecError::NotInDialect(id))
            }

            fn decode(payload: &::data_core::mavlink::mavio::protocol::Payload) -> Result<Self, ::data_core::mavlink::mavio::prelude::SpecError> {
                #(
                    if <#types as ::data_core::mavlink::hal::description::DescribeMessages>::MESSAGES
                        .iter()
                        .any(|m| m.id == payload.id())
                    {
                        return #types::try_from(payload).map(Self::#idents);
                    }
                )*

                Err(::data_core::mavlink::mavio::prelude::SpecError::NotInDialect(payload.id()))
            }

            fn spec() -> &'static ::data_core::mavlink::mavio::protocol::DialectSpec {
                const DIALECT_SPEC: ::data_core::mavlink::mavio::protocol::DialectSpec = ::data_core::mavlink::mavio::protocol::DialectSpec::new(
                    #name,
                    core::option::Option::Some(#id),
                    core::option::Option::Some(#version),
                    &::data_core::mavlink::hal::macro_util::into_mavio_infos::<
                        { <#ident as ::data_core::mavlink::hal::description::DescribeMessages>::MESSAGES.len() },
                    >(<#ident as ::data_core::mavlink::hal::description::DescribeMessages>::MESSAGES),
                );
                &DIALECT_SPEC
            }
        }
    }
}

fn generate_describe_messages_impl(
    ident: &Ident,
    variants: &[MavlinkDialectVariant],
) -> TokenStream {
    let types = variants
        .iter()
        .map(|variant| {
            let field = variant
                .fields
                .fields
                .first()
                .expect("should have exactly one field since it's a newtype");

            &field.ty
        })
        .collect::<Vec<_>>();

    quote! {
        impl ::data_core::mavlink::hal::description::DescribeMessages for #ident {
            const MESSAGES: &[::data_core::mavlink::hal::description::Message] = &::data_core::macro_util::concat_slices::<_, {
                    #(<#types as ::data_core::mavlink::hal::description::DescribeMessages>::MESSAGES.len() +)* 0
                }>(
                    &[#(<#types as ::data_core::mavlink::hal::description::DescribeMessages>::MESSAGES,)*]
                );
        }

        const _: () = ::data_core::mavlink::hal::macro_util::check_unique_ids(
            <#ident as ::data_core::mavlink::hal::description::DescribeMessages>::MESSAGES,
        );
    }
}

fn generate_message_impl(ident: &Ident, variants: &[MavlinkDialectVariant]) -> TokenStream {
    let idents = variants
        .iter()
        .map(|variant| &variant.ident)
        .collect::<Vec<_>>();

    quote! {
        impl ::data_core::mavlink::mavio::protocol::MessageSpec for #ident {
            fn id(&self) -> ::data_core::mavlink::mavio::protocol::MessageId {
                match self {
                    #(#ident::#idents(message) => message.id(),)*
                }
            }

            fn min_supported_mavlink_version(&self) -> ::data_core::mavlink::mavio::prelude::MavLinkVersion {
                match self {
                    #(#ident::#idents(message) => message.min_supported_mavlink_version(),)*
                }
            }

            fn crc_extra(&self) -> ::data_core::mavlink::mavio::protocol::CrcExtra {
                match self {
                    #(#ident::#idents(message) => message.crc_extra(),)*
                }
            }
        }

        impl ::data_core::mavlink::mavio::protocol::IntoPayload for #ident {
            fn encode(
                &self,
                version: ::data_core::mavlink::mavio::prelude::MavLinkVersion,
            ) -> Result<::data_core::mavlink::mavio::protocol::Payload, ::data_core::mavlink::mavio::prelude::SpecError> {
                match self {
                    #(#ident::#idents(message) => message.encode(version),)*
                }
            }
        }

        impl ::data_core::mavlink::mavio::Message for #ident {}
    }
}
