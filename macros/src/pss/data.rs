use darling::{FromDeriveInput, FromField, FromVariant, ast::Fields, util::Flag};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, LitStr};

use crate::util::{GetUnit, Unit};
use darling::util::SpannedValue;
use darling::{FromMeta, ToTokens};

#[derive(FromMeta, Clone, Copy)]
#[darling(rename_all = "snake_case")]
enum DataKind {
    DigitalActuator,
    DprActuator,
    FractionalActuator,
    AnalogSensor,
    DigitalSensor,
}

/// Registry mapping a type's last path segment to its inferred [`DataKind`].
///
/// When a field or variant annotated with `#[pss(name = "...")]` uses one of these types,
/// the `kind` attribute may be omitted and will be inferred automatically from this registry.
const TYPE_KIND_REGISTRY: &[(&str, DataKind)] = &[
    ("NormallyClosedValve", DataKind::DigitalActuator),
    ("NormallyOpenValve", DataKind::DigitalActuator),
    ("DigitalOut", DataKind::DigitalActuator),
    ("InvertedDigitalOut", DataKind::DigitalActuator),
    // `Celsius` is an analog sensor — additionally requires a `#[unit("...")]` attribute
    ("Celsius", DataKind::AnalogSensor),
    ("DigitalIn", DataKind::DigitalSensor),
];

/// Infer a [`DataKind`] from the last path segment of `ty` using [`TYPE_KIND_REGISTRY`].
fn infer_kind(ty: &syn::Type) -> Option<DataKind> {
    let syn::Type::Path(type_path) = ty else {
        return None;
    };
    let ident = type_path.path.segments.last()?.ident.to_string();
    TYPE_KIND_REGISTRY
        .iter()
        .find(|(name, _)| *name == ident)
        .map(|(_, kind)| *kind)
}

struct DataKindWithUnit<'a>(&'a SpannedValue<DataKind>, Option<&'a Unit>);

impl ToTokens for DataKindWithUnit<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let data = quote!(::data_core::data::datapoint);
        let ident = match (self.0.as_ref(), self.1) {
            (DataKind::DigitalActuator, _) => quote!(Actuator(#data::ActuatorKind::Digital)),
            (DataKind::DprActuator, _) => quote!(Actuator(#data::ActuatorKind::DPR)),
            (DataKind::FractionalActuator, _) => quote!(Actuator(#data::ActuatorKind::Fractional)),
            (DataKind::AnalogSensor, Some(unit)) => {
                quote!(Sensor(#data::SensorKind::Analog { unit: #unit }))
            }
            (DataKind::AnalogSensor, _) => {
                let err = darling::Error::custom(
                    "Analog sensor requires an `#[unit(\"...\")]` attribute",
                )
                .with_span(&self.0.span());
                tokens.extend(err.write_errors());
                return;
            }
            (DataKind::DigitalSensor, _) => quote!(Sensor(#data::SensorKind::Digital)),
        };
        tokens.extend(quote!(#data::DatapointKind::#ident));
    }
}

#[derive(FromField)]
#[darling(attributes(pss))]
pub struct PssDataField {
    /// Unique name of the PSS datapoint
    ///
    /// **Important**: Only fields which have a `#[pss(...)]` attribute are considered by this macro
    name: Option<LitStr>,
    /// User-facing label of the PSS datapoint
    ///
    /// **Note**: This field is required once a `#[pss(name = ...)]` attribute is set
    label: Option<LitStr>,
    /// Type of the PSS datapoint
    ///
    /// **Note**: This field is required once a `#[pss(name = ...)]` attribute is set,
    /// unless the type is listed in [`TYPE_KIND_REGISTRY`] (e.g. `NormallyClosedValve`,
    /// `NormallyOpenValve`, `DigitalOut`, `InvertedDigitalOut`), in which case it is inferred.
    kind: Option<SpannedValue<DataKind>>,
    /// Boolean flag whether this field should be flattened
    flatten: Flag,

    ident: Option<Ident>,
    ty: syn::Type,
}

#[derive(FromVariant)]
#[darling(attributes(pss))]
pub struct PssDataVariant {
    /// Unique name of the PSS datapoint
    ///
    /// **Important**: Only variants which have a `#[pss(...)]` attribute are considered by this macro
    name: Option<LitStr>,
    /// User-facing label of the PSS datapoint
    ///
    /// **Note**: This field is required once a `#[pss(name = ...)]` attribute is set
    label: Option<LitStr>,
    /// Type of the PSS datapoint
    ///
    /// **Note**: This field is required once a `#[pss(name = ...)]` attribute is set,
    /// unless the type is listed in [`TYPE_KIND_REGISTRY`] (e.g. `NormallyClosedValve`,
    /// `NormallyOpenValve`, `DigitalOut`, `InvertedDigitalOut`), in which case it is inferred.
    kind: Option<SpannedValue<DataKind>>,

    /// Boolean flag whether this variant should be flattened
    flatten: Flag,

    ident: Ident,
    fields: Fields<PssDataField>,
}

#[derive(FromDeriveInput)]
#[darling(
    supports(enum_named, enum_newtype, enum_unit, struct_named, struct_newtype),
    attributes(pss)
)]
pub struct PssData {
    /// Unique name of the PSS datapoint
    ///
    /// **Important**: This can only be set on newtype enums and structs
    name: Option<LitStr>,
    /// User-facing label of the PSS datapoint
    ///
    /// **Important**: This can only be set on newtype enums and structs
    label: Option<LitStr>,
    /// Boolean flag whether this newtype should be flattened
    flatten: Flag,

    ident: Ident,
    data: darling::ast::Data<PssDataVariant, PssDataField>,
}

pub fn handle(pss_data: PssData) -> TokenStream {
    let PssData {
        ident,
        name,
        label,
        flatten,
        ..
    } = pss_data;

    let impl_tokenstream = if pss_data.data.is_enum() {
        let mut errors = darling::Error::accumulator();
        if flatten.is_present() {
            errors.push(
                darling::Error::custom(
                    "Attribute `#[pss(flatten)]` cannot be set on enum container",
                )
                .with_span(&flatten.span()),
            );
        }
        if name.is_some() {
            errors.push(
                darling::Error::custom(
                    "Attribute `#[pss(name = ...)]` cannot be set on enum container",
                )
                .with_span(&ident),
            );
        }
        if label.is_some() {
            errors.push(
                darling::Error::custom(
                    "Attribute `#[pss(label = ...)]` cannot be set on enum container",
                )
                .with_span(&ident),
            );
        }

        if let Err(err) = errors.finish() {
            return err.write_errors();
        }

        let variants = pss_data
            .data
            .take_enum()
            .expect("should be enum")
            .into_iter()
            // Filter out unit fields as there can't be any data for them.
            .filter(|variant| !variant.fields.is_unit())
            .collect::<Vec<_>>();
        handle_enum(&ident, &variants)
    } else {
        let fields = pss_data.data.take_struct().expect("should be struct");
        handle_struct(&ident, flatten, name.as_ref(), label.as_ref(), &fields)
    };

    quote! {
        #impl_tokenstream

        const _: () = ::data_core::data::macro_util::check_unique_names(
            <#ident as ::data_core::data::datapoint::DatapointHolder>::DATAPOINTS
        );

        const _: () = ::data_core::data::macro_util::check_name_len(
            <#ident as ::data_core::data::datapoint::DatapointHolder>::DATAPOINTS
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

            let (datapoints, len) = match get_datapoints(ty, variant.flatten, variant.name.as_ref(), variant.label.as_ref()) {
                Ok(Some((datapoints, len))) => (datapoints, len),
                Ok(None) => return None,
                Err(err) => {
                    errors.push(err);
                    return None;
                },
            };

            let value = if let Some(name) = &variant.name {
                quote! {
                    #ident::#variant_ident(value) => vec![
                        ::data_core::data::datapoint::NamedDatapointValue {
                            name: #name,
                            value: ::data_core::data::datapoint::IntoDatapointValue::into_value(*value)
                        }
                    ]
                }
            } else {
                quote! {
                    #ident::#variant_ident(value) => <#ty as ::data_core::data::datapoint::DatapointHolder>::get_datapoint_values(value)
                }
            };

            Some((datapoints, len, value))
        } else if variant.fields.is_struct() {
            if variant.name.is_some() {
                errors.push(darling::Error::custom("Attribute `#[pss(name = ...)]` cannot be used on struct enum variants").with_span(&variant_ident));
            }

            let (idents, datapoints, lengths, values) = variant.fields.iter().filter_map(|field| {
                let field_ident = field.ident.as_ref().expect("non-newtype field should always have a name");
                let ty = &field.ty;

                let (datapoints, len) = match get_datapoints(ty, field.flatten, field.name.as_ref(), field.label.as_ref()) {
                    Ok(Some((datapoints, len))) => (datapoints, len),
                    Ok(None) => return None,
                    Err(err) => {
                        errors.push(err);
                        return None;
                    },
                };

                let value = if let Some(name) = &field.name {
                    quote! {
                        values.extend(vec![
                            ::data_core::data::datapoint::NamedDatapointValue {
                                name: #name,
                                value: ::data_core::data::datapoint::IntoDatapointValue::into_value(#field_ident)
                            }
                        ]);
                    }
                } else {
                    quote! {
                        values.extend(<#ty as ::data_core::data::datapoint::DatapointHolder>::get_datapoint_values(&#field_ident));
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
        impl ::data_core::data::datapoint::DatapointHolder for #ident {
            const DATAPOINTS: &[::data_core::data::datapoint::DatapointInformation] = #datapoints;

            fn get_datapoint_values(&self) -> std::vec::Vec<::data_core::data::datapoint::NamedDatapointValue> {
                match self {
                    #(#values,)*
                    _ => std::vec::Vec::new()
                }
            }
        }
    }
}

fn handle_struct(
    ident: &Ident,
    flatten: Flag,
    name: Option<&LitStr>,
    label: Option<&LitStr>,
    fields: &Fields<PssDataField>,
) -> TokenStream {
    let (datapoints, values) = if fields.is_newtype() {
        let field = fields
            .fields
            .first()
            .expect("newtype struct should have exactly one field");

        let mut errors = darling::Error::accumulator();
        if flatten.is_present() {
            errors.push(darling::Error::custom("Attribute `#[pss(flatten)]` cannot be used on newtype structs, use this attribute on the container-level instead!").with_span(&flatten.span()));
        }
        if field.name.is_some() {
            errors.push(darling::Error::custom("Attribute `#[pss(name = ...)]` cannot be used on newtype structs, use this attribute on the container-level instead!").with_span(&field.name));
        }
        if field.label.is_some() {
            errors.push(darling::Error::custom("Attribute `#[pss(label = ...)]` cannot be used on newtype structs, use this attribute on the container-level instead!").with_span(&field.name));
        }

        if let Err(err) = errors.finish() {
            return err.write_errors();
        }

        let ty = &field.ty;

        let datapoints = match get_datapoints(ty, flatten, name, label) {
            Ok(datapoints) => datapoints,
            Err(err) => return err.write_errors(),
        };

        if let Some((datapoints, _)) = datapoints {
            let value = if let Some(name) = name {
                quote! {
                    values.extend(vec![
                        ::data_core::data::datapoint::NamedDatapointValue {
                            name: #name,
                            value: ::data_core::data::datapoint::IntoDatapointValue::into_value(self.0)
                        }
                    ]);
                }
            } else {
                quote! {
                    values.extend(<#ty as ::data_core::data::datapoint::DatapointHolder>::get_datapoint_values(&self.0));
                }
            };
            (datapoints, vec![value])
        } else {
            (quote!(&[]), vec![])
        }
    } else {
        let mut errors = darling::Error::accumulator();
        let (datapoints, lengths, values) = fields.fields.iter().filter_map(|field| {
            let field_ident = field.ident.as_ref().expect("non-newtype field should always have a name");
            let ty = &field.ty;

            let (datapoints, len) = match get_datapoints(ty, field.flatten, field.name.as_ref(), field.label.as_ref()) {
                Ok(Some((datapoints, len))) => (datapoints, len),
                Ok(None) => return None,
                Err(err) => {
                    errors.push(err);
                    return None;
                },
            };

            let value = if let Some(name) = &field.name {
                quote! {
                    values.extend(vec![
                        ::data_core::data::datapoint::NamedDatapointValue {
                            name: #name,
                            value: ::data_core::data::datapoint::IntoDatapointValue::into_value(self.#field_ident)
                        }
                    ]);
                }
            } else {
                quote! {
                    values.extend(<#ty as ::data_core::data::datapoint::DatapointHolder>::get_datapoint_values(&self.#field_ident));
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
        impl ::data_core::data::datapoint::DatapointHolder for #ident {
            const DATAPOINTS: &[::data_core::data::datapoint::DatapointInformation] = #datapoints;

            fn get_datapoint_values(&self) -> std::vec::Vec<::data_core::data::datapoint::NamedDatapointValue> {
                let mut values = std::vec::Vec::new();
                #(#values;)*
                values
            }
        }
    }
}

/// Get the datapoints for a single field and an expression which evaluates
/// to the length of all returned datapoints.
///
/// Returns [`Option`] with tuple containing token stream for slice with datapoint information and token stream
/// for the length of said slice. If the option is [`None`], no token streams need to be added to the output
/// for this field.
fn get_datapoints(
    ty: &syn::Type,
    flatten: Flag,
    name: Option<&LitStr>,
    label: Option<&LitStr>,
) -> darling::Result<Option<(TokenStream, TokenStream)>> {
    if flatten.is_present() && (name.is_some() || label.is_some()) {
        Err(darling::Error::custom("Attribute `#[pss(flatten)]` cannot be combined with `#[pss(name = ...)]` or `#[pss(label = ...)]`").with_span(&flatten.span()))?;
    }

    if !flatten.is_present() && name.is_none() {
        return Ok(None);
    }

    let Some(name) = name else {
        if let Some(label) = label {
            return Err(darling::Error::custom(
                "Attribute `#[pss(label = ...)]` cannot be used without `#[pss(name = ...)]`",
            )
            .with_span(&label.span()));
        }

        let datapoints = quote! {
            <#ty as ::data_core::data::datapoint::DatapointHolder>::DATAPOINTS
        };
        let datapoints_len = quote!(#datapoints.len());
        return Ok(Some((datapoints, datapoints_len)));
    };

    let inferred: Option<SpannedValue<DataKind>> = kind
        .is_none()
        .then(|| infer_kind(ty))
        .flatten()
        .map(|k| SpannedValue::new(k, proc_macro2::Span::call_site()));
    let effective_kind = kind.or(inferred.as_ref());
    let Some((kind, label)) = effective_kind
        .map(|kind| DataKindWithUnit(kind, unit))
        .zip(label)
    else {
        return Err(darling::Error::custom(
            "Attribute `#[pss(label = ...)]` is required when `#[pss(name = ...)]` is set. \
             Also set `#[pss(kind = ...)]` unless the type is listed in TYPE_KIND_REGISTRY.",
        )
        .with_span(name));
    };

    let datapoints = quote! {
        &[
            ::data_core::data::datapoint::DatapointInformation {
                kind: <#ty as ::data_core::data::datapoint::IntoDatapointValue>::KIND,
                label: #label,
                name: #name
            }
        ]
    };
    Ok(Some((datapoints, quote!(1))))
}

/// Concatenate a list of datapoints and their corresponding lengths
fn concatenate_datapoints(datapoints: &[TokenStream], lengths: &[TokenStream]) -> TokenStream {
    match datapoints.len() {
        0 => quote!(&[]),
        1 => quote! { #(#datapoints)* },
        _ => {
            // concatenate slices without storing intermediates
            datapoints.iter().zip(lengths).fold((quote!(&[]), quote!(0)), |(acc, acc_len), (info, length)| {
                let next = quote! {
                    &::data_core::macro_util::concat_slices::<_, { #acc_len + #length }>(&[#acc, #info])
                };
                (next, quote! { #acc_len + #length })
            }).0
        }
    }
}
