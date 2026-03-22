use darling::{
    FromDeriveInput, FromField, FromMeta, FromVariant,
    ast::Fields,
    util::{Flag, SpannedValue},
};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Ident, LitStr};

use crate::util::{GetUnit, Unit};

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
#[darling(attributes(pss), forward_attrs(unit))]
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
    attrs: Vec<syn::Attribute>,
}

#[derive(FromVariant)]
#[darling(attributes(pss), forward_attrs(unit))]
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
    attrs: Vec<syn::Attribute>,
}

#[derive(FromDeriveInput)]
#[darling(
    supports(enum_named, enum_newtype, struct_named, struct_newtype),
    attributes(pss),
    forward_attrs(unit)
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
    /// Type of the PSS datapoint
    ///
    /// **Note**: This can only be set on newtype enums and structs
    kind: Option<SpannedValue<DataKind>>,
    /// Boolean flag whether this newtype should be flattened
    flatten: Flag,

    ident: Ident,
    data: darling::ast::Data<PssDataVariant, PssDataField>,
    attrs: Vec<syn::Attribute>,
}

pub fn handle(pss_data: PssData) -> TokenStream {
    let PssData {
        ident,
        name,
        label,
        kind,
        flatten,
        ..
    } = pss_data;
    let unit = match pss_data.attrs.get_unit() {
        Ok(unit) => unit,
        Err(err) => return err.write_errors(),
    };

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
        if kind.is_some() {
            errors.push(
                darling::Error::custom(
                    "Attribute `#[pss(kind = ...)]` cannot be set on enum container",
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
            flatten,
            name.as_ref(),
            label.as_ref(),
            kind.as_ref(),
            unit.as_ref(),
            &fields,
        )
    };

    quote! {
        #impl_tokenstream

        const _: () = ::data_core::data::macro_util::check_unique_names(
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
            let unit = match variant.attrs.get_unit() {
                Ok(unit) => unit,
                Err(err) => {
                    errors.push(err);
                    return None;
                },
            };

            let (datapoints, len) = match get_datapoints(ty, variant.flatten, variant.name.as_ref(), variant.label.as_ref(), variant.kind.as_ref(), unit.as_ref()) {
                Ok(Some((datapoints, len))) => (datapoints, len),
                Ok(None) => return None,
                Err(err) => {
                    errors.push(err);
                    return None;
                },
            };

            let value = if let Some(name) = &variant.name {
                quote! {
                    #ident::#variant_ident(value) => vec![::data_core::data::datapoint::NamedDatapointValue { name: #name, value: (*value).into() }]
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
            if variant.kind.is_some() {
                errors.push(darling::Error::custom("Attribute `#[pss(kind = ...)]` cannot be used on struct enum variants").with_span(&variant_ident));
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

                let (datapoints, len) = match get_datapoints(ty, field.flatten, field.name.as_ref(), field.label.as_ref(), field.kind.as_ref(), unit.as_ref()) {
                    Ok(Some((datapoints, len))) => (datapoints, len),
                    Ok(None) => return None,
                    Err(err) => {
                        errors.push(err);
                        return None;
                    },
                };

                let value = if let Some(name) = &field.name {
                    quote! {
                        values.extend(vec![::data_core::data::datapoint::NamedDatapointValue { name: #name, value: #field_ident.into() }]);
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
    kind: Option<&SpannedValue<DataKind>>,
    unit: Option<&Unit>,
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
        if let Some(kind) = &field.kind {
            errors.push(darling::Error::custom("Attribute `#[pss(kind = ...)]` cannot be used on newtype structs, use this attribute on the container-level instead!").with_span(&kind.span()));
        }

        if let Err(err) = errors.finish() {
            return err.write_errors();
        }

        let ty = &field.ty;

        let datapoints = match get_datapoints(ty, flatten, name, label, kind, unit) {
            Ok(datapoints) => datapoints,
            Err(err) => return err.write_errors(),
        };

        if let Some((datapoints, _)) = datapoints {
            let value = if let Some(name) = name {
                quote! {
                    values.extend(vec![::data_core::data::datapoint::NamedDatapointValue { name: #name, value: (self.0).into() }]);
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
            let unit = match field.attrs.get_unit() {
                Ok(unit) => unit,
                Err(err) => {
                    errors.push(err);
                    return None;
                },
            };

            let (datapoints, len) = match get_datapoints(ty, field.flatten, field.name.as_ref(), field.label.as_ref(), field.kind.as_ref(), unit.as_ref()) {
                Ok(Some((datapoints, len))) => (datapoints, len),
                Ok(None) => return None,
                Err(err) => {
                    errors.push(err);
                    return None;
                },
            };

            let value = if let Some(name) = &field.name {
                quote! {
                    values.extend(vec![::data_core::data::datapoint::NamedDatapointValue { name: #name, value: (self.#field_ident).into() }]);
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
    kind: Option<&SpannedValue<DataKind>>,
    unit: Option<&Unit>,
) -> darling::Result<Option<(TokenStream, TokenStream)>> {
    if flatten.is_present() && (kind.is_some() || name.is_some() || label.is_some()) {
        Err(darling::Error::custom("Attribute `#[pss(flatten)]` cannot be combined with `#[pss(name = ...)]`, `#[pss(label = ...)]` or `#[pss(kind = ...)]`").with_span(&flatten.span()))?;
    }

    if !flatten.is_present() && kind.is_none() && name.is_none() {
        return Ok(None);
    }

    let Some(name) = name else {
        let mut errors = darling::Error::accumulator();
        if let Some(label) = label {
            errors.push(
                darling::Error::custom(
                    "Attribute `#[pss(label = ...)]` cannot be used without `#[pss(name = ...)]`",
                )
                .with_span(&label.span()),
            );
        }
        if let Some(kind) = kind {
            errors.push(
                darling::Error::custom(
                    "Attribute `#[pss(kind = ...)]` cannot be used without `#[pss(name = ...)]`",
                )
                .with_span(&kind.span()),
            );
        }

        errors.finish()?;

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
        .with_span(name))?;
    };

    let datapoints = quote! {
        &[
            ::data_core::data::datapoint::DatapointInformation {
                kind: #kind,
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
        0 => quote!([]),
        1 => quote! { #(#datapoints)* },
        _ => {
            // concatenate slices without storing intermediates
            datapoints.iter().zip(lengths).fold((quote!(&[]), quote!(0)), |(acc, acc_len), (info, length)| {
                let next = quote! {
                    &::data_core::data::macro_util::concat_slices::<_, { #acc_len + #length }>(&[#acc, #info])
                };
                (next, quote! { #acc_len + #length })
            }).0
        }
    }
}
