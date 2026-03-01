use darling::{FromDeriveInput, FromField, FromMeta, FromVariant, ast::Fields, util::SpannedValue};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Ident, LitStr};

use crate::util::{GetUnit, Unit};

#[derive(FromMeta)]
#[darling(rename_all = "snake_case")]
enum DataType {
    #[darling(rename = "dpr")]
    #[allow(clippy::upper_case_acronyms)]
    DPR,
    InteractableBinary,
    Binary,
    Value,
}

impl ToTokens for DataType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = match self {
            DataType::DPR => quote!(DPR),
            DataType::InteractableBinary => quote!(InteractableBinary),
            DataType::Binary => quote!(Binary),
            DataType::Value => quote!(Value),
        };
        tokens.extend(ident);
    }
}

#[derive(FromField)]
#[darling(attributes(pss), forward_attrs(unit))]
pub struct PssDataField {
    /// Unique id of the PSS datapoint
    ///
    /// **Important**: Only fields which have a `#[pss(id = ...)]` attribute are considered by this macro
    id: Option<LitStr>,
    /// Type of the PSS datapoint
    ///
    /// **Note**: This field is required once a `#[pss(id = ...)]` attribute is set
    #[darling(rename = "type")]
    data_type: Option<SpannedValue<DataType>>,

    ident: Option<Ident>,
    ty: syn::Type,
    attrs: Vec<syn::Attribute>,
}

#[derive(FromVariant)]
#[darling(attributes(pss), forward_attrs(unit))]
pub struct PssDataVariant {
    /// Unique id of the PSS datapoint
    ///
    /// **Important**: Only variants which have a `#[pss(id = ...)]` attribute are considered by this macro
    id: Option<LitStr>,
    /// Type of the PSS datapoint
    ///
    /// **Note**: This field is required once a `#[pss(id = ...)]` attribute is set
    #[darling(rename = "type")]
    data_type: Option<SpannedValue<DataType>>,

    ident: Ident,
    fields: Fields<PssDataField>,
    attrs: Vec<syn::Attribute>,
}

#[derive(FromDeriveInput)]
#[darling(
    supports(enum_named, enum_newtype, struct_named, struct_newtype),
    attributes(pss),
    forward_attrs(unit)
)]
pub struct PssData {
    /// Unique id of the PSS datapoint
    ///
    /// **Important**: This can only be set on newtype enums and structs
    id: Option<LitStr>,
    /// Type of the PSS datapoint
    ///
    /// **Note**: This can only be set on newtype enums and structs
    #[darling(rename = "type")]
    data_type: Option<SpannedValue<DataType>>,

    ident: Ident,
    data: darling::ast::Data<PssDataVariant, PssDataField>,
    attrs: Vec<syn::Attribute>,
}

pub fn handle(pss_data: PssData) -> TokenStream {
    let PssData {
        ident,
        id,
        data_type,
        ..
    } = pss_data;
    let unit = match pss_data.attrs.get_unit() {
        Ok(unit) => unit,
        Err(err) => return err.write_errors(),
    };

    let impl_tokenstream = if pss_data.data.is_enum() {
        let mut errors = darling::Error::accumulator();
        if id.is_some() {
            errors.push(
                darling::Error::custom(
                    "Attribute `#[pss(id = ...)]` cannot be set on enum container",
                )
                .with_span(&ident),
            );
        }
        if data_type.is_some() {
            errors.push(
                darling::Error::custom(
                    "Attribute `#[pss(type = ...)]` cannot be set on enum container",
                )
                .with_span(&ident),
            );
        }
        if let Err(err) = errors.finish() {
            return err.write_errors();
        }

        let variants = pss_data.data.take_enum().expect("should be enum");
        handle_enum(&ident, &variants)
    } else {
        let fields = pss_data.data.take_struct().expect("should be struct");
        handle_struct(
            &ident,
            id.as_ref(),
            data_type.as_ref(),
            unit.as_ref(),
            &fields,
        )
    };

    quote! {
        #impl_tokenstream

        const _: () = ::data_core::pss::lib::macro_util::check_unique_ids(
            <#ident as ::data_core::pss::lib::data::DatapointHolder>::DATAPOINTS
        );
    }
}

fn handle_enum(ident: &Ident, variants: &[PssDataVariant]) -> TokenStream {
    let mut errors = darling::Error::accumulator();

    let (datapoints, lengths, values) = variants.iter().filter_map(|variant| {
        let variant_ident = &variant.ident;
        if variant.fields.is_newtype() {
            let field = variant.fields.fields.first().expect("newtype variant should have exactly one field");
            let ty = &field.ty;
            let unit = match variant.attrs.get_unit() {
                Ok(unit) => unit,
                Err(err) => {
                    errors.push(err);
                    return None;
                },
            };

            let (datapoints, len) = match get_datapoints(ty, variant.id.as_ref(), variant.data_type.as_ref(), unit.as_ref()) {
                Ok((datapoints, len)) => (datapoints, len),
                Err(err) => {
                    errors.push(err);
                    return None;
                },
            };

            let value = if let Some(id) = &variant.id {
                quote! {
                    #ident::#variant_ident(value) => vec![::data_core::pss::lib::data::DatapointValue { id: #id, data: (*value).into() }]
                }
            } else {
                quote! {
                    #ident::#variant_ident(value) => <#ty as ::data_core::pss::lib::data::DatapointHolder>::get_datapoint_values(value)
                }
            };

            Some((datapoints, len, value))
        } else if variant.fields.is_struct() {
            if variant.id.is_some() {
                errors.push(darling::Error::custom("Attribute `#[pss(id = ...)]` cannot be used on struct enum variants").with_span(&variant_ident));
            }
            if variant.data_type.is_some() {
                errors.push(darling::Error::custom("Attribute `#[pss(type = ...)]` cannot be used on struct enum variants").with_span(&variant_ident));
            }

            let (idents, datapoints, lengths, values) = variant.fields.iter().filter_map(|field| {
                let field_ident = field.ident.as_ref().expect("non-newtype field should always have a name");
                let ty = &field.ty;
                let unit = match field.attrs.get_unit() {
                    Ok(unit) => unit,
                    Err(err) => {
                        errors.push(err);
                        return None;
                    },
                };

                let (datapoints, len) = match get_datapoints(ty, field.id.as_ref(), field.data_type.as_ref(), unit.as_ref()) {
                    Ok((datapoints, len)) => (datapoints, len),
                    Err(err) => {
                        errors.push(err);
                        return None;
                    },
                };

                let value = if let Some(id) = &field.id {
                    quote! {
                        values.extend(vec![::data_core::pss::lib::data::DatapointValue { id: #id, data: #field_ident.into() }]);
                    }
                } else {
                    quote! {
                        values.extend(<#ty as ::data_core::pss::lib::data::DatapointHolder>::get_datapoint_values(&#field_ident));
                    }
                };

                Some((field_ident, datapoints, len, value))
            }).collect::<(Vec<_>, Vec<_>, Vec<_>, Vec<_>)>();

            let datapoints = concatenate_datapoints(&datapoints, &lengths);
            let length = quote!(#(#lengths + )* 0);
            let value = quote! {
              #ident::#variant_ident { #(#idents),* } => {
                  let mut values = std::vec::Vec::new();
                  #(#values;)*
                  values
              }
            };

            Some((datapoints, length, value))

        } else {
            unreachable!("we only support newtype and named enums")
        }

    }).collect::<(Vec<_>, Vec<_>, Vec<_>)>();

    if let Err(err) = errors.finish() {
        return err.write_errors();
    }

    let datapoints = concatenate_datapoints(&datapoints, &lengths);

    quote! {
        impl ::data_core::pss::lib::data::DatapointHolder for #ident {
            const DATAPOINTS: &[::data_core::pss::lib::data::DatapointInformation] = #datapoints;

            fn get_datapoint_values(&self) -> std::vec::Vec<::data_core::pss::lib::data::DatapointValue> {
                match self {
                    #(#values),*
                }
            }
        }
    }
}

fn handle_struct(
    ident: &Ident,
    id: Option<&LitStr>,
    data_type: Option<&SpannedValue<DataType>>,
    unit: Option<&Unit>,
    fields: &Fields<PssDataField>,
) -> TokenStream {
    let (datapoints, values) = if fields.is_newtype() {
        let field = fields
            .fields
            .first()
            .expect("newtype struct should have exactly one field");

        let mut errors = darling::Error::accumulator();
        if field.id.is_some() {
            errors.push(darling::Error::custom("Attribute `#[pss(id = ...)]` cannot be used on newtype structs, use this attribute on the container-level instead!").with_span(&field.id));
        }
        if let Some(data_type) = &field.data_type {
            errors.push(darling::Error::custom("Attribute `#[pss(type = ...)]` cannot be used on newtype structs, use this attribute on the container-level instead!").with_span(&data_type.span()));
        }

        if let Err(err) = errors.finish() {
            return err.write_errors();
        }

        let ty = &field.ty;

        let datapoints = match get_datapoints(ty, id, data_type, unit) {
            Ok((datapoints, _)) => datapoints,
            Err(err) => return err.write_errors(),
        };

        let value = if let Some(id) = id {
            quote! {
                values.extend(vec![::data_core::pss::lib::data::DatapointValue { id: #id, data: (self.0).into() }]);
            }
        } else {
            quote! {
                values.extend(<#ty as ::data_core::pss::lib::data::DatapointHolder>::get_datapoint_values(&self.0));
            }
        };
        (datapoints, vec![value])
    } else {
        let mut errors = darling::Error::accumulator();
        let (datapoints, lengths, values) = fields.fields.iter().filter_map(|field| {
            let field_ident = field.ident.as_ref().expect("non-newtype field should always have a name");
            let ty = &field.ty;
            let unit = match field.attrs.get_unit() {
                Ok(unit) => unit,
                Err(err) => {
                    errors.push(err);
                    return None;
                },
            };

            let (datapoints, len) = match get_datapoints(ty, field.id.as_ref(), field.data_type.as_ref(), unit.as_ref()) {
                Ok((datapoints, len)) => (datapoints, len),
                Err(err) => {
                    errors.push(err);
                    return None;
                },
            };

            let value = if let Some(id) = &field.id {
                quote! {
                    values.extend(vec![::data_core::pss::lib::data::DatapointValue { id: #id, data: (self.#field_ident).into() }]);
                }
            } else {
                quote! {
                    values.extend(<#ty as ::data_core::pss::lib::data::DatapointHolder>::get_datapoint_values(&self.#field_ident));
                }
            };

            Some((datapoints, len, value))
        }).collect::<(Vec<_>, Vec<_>, Vec<_>)>();

        if let Err(err) = errors.finish() {
            return err.write_errors();
        }

        let datapoints = concatenate_datapoints(&datapoints, &lengths);
        (datapoints, values)
    };

    quote! {
        impl ::data_core::pss::lib::data::DatapointHolder for #ident {
            const DATAPOINTS: &[::data_core::pss::lib::data::DatapointInformation] = #datapoints;

            fn get_datapoint_values(&self) -> std::vec::Vec<::data_core::pss::lib::data::DatapointValue> {
                let mut values = std::vec::Vec::new();
                #(#values;)*
                values
            }
        }
    }
}

/// Get the datapoints for a single field and an expression which evaluates
/// to the length of all returned datapoints
fn get_datapoints(
    ty: &syn::Type,
    id: Option<&LitStr>,
    data_type: Option<&SpannedValue<DataType>>,
    unit: Option<&Unit>,
) -> darling::Result<(TokenStream, TokenStream)> {
    let Some(id) = id else {
        if let Some(data_type) = data_type {
            Err(darling::Error::custom(
                "Attribute `#[pss(type = ...)]` cannot be used without `#[pss(id = ...)]`",
            )
            .with_span(&data_type.span()))?;
        }
        let datapoints = quote! {
            <#ty as ::data_core::pss::lib::data::DatapointHolder>::DATAPOINTS
        };
        let datapoints_len = quote!(#datapoints.len());
        return Ok((datapoints, datapoints_len));
    };
    let Some(data_type) = data_type.map(std::convert::AsRef::as_ref) else {
        return Err(darling::Error::custom(
            "Attribute `#[pss(type = ...)]` is required when `#[pss(id = ...)]` is set",
        )
        .with_span(id))?;
    };
    let unit = if let Some(unit) = unit {
        quote!(Some(#unit))
    } else {
        quote!(None)
    };

    let datapoints = quote! {
        &[
            ::data_core::pss::lib::data::DatapointInformation {
                unit: #unit,
                data_type: ::data_core::pss::lib::data::DatapointType::#data_type,
                id: #id
            }
        ]
    };
    Ok((datapoints, quote!(1)))
}

/// Concatenate a list of datapoints and their corresponding lengths
fn concatenate_datapoints(datapoints: &[TokenStream], lengths: &[TokenStream]) -> TokenStream {
    match datapoints.len() {
        0 => quote!([]),
        1 => quote! { #(#datapoints)* },
        _ => {
            // concatenate slices without storing intermediates
            datapoints.iter().zip(lengths).fold((quote!(&[]), quote!(0)), |(acc, acc_len), (info, length)| {
                let next = quote! {
                    &::data_core::pss::lib::macro_util::concat_slices::<_, { #acc_len + #length }>(&[#acc, #info])
                };
                (next, quote! { #acc_len + #length })
            }).0
        }
    }
}
