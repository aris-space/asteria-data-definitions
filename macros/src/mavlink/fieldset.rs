use darling::{FromDeriveInput, FromField, ast::Fields, util::Flag};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use stringcase::snake_case;
use syn::{Ident, LitStr, spanned::Spanned};

use crate::util::{GetDocs, GetUnit};

#[derive(FromField)]
#[darling(attributes(mavlink), forward_attrs(doc, unit))]
pub struct MavlinkFieldSetField {
    /// Boolean flag whether the field should be flattened
    flatten: Flag,
    /// Boolean flag whether the field should be refined
    refine: Flag,
    /// Optional name override for the field. If left empty, for newtype structs
    /// the snake_case struct name is used as the default name and for struct fields
    /// the snake_case field name is used as the default name.
    ///
    /// **Note**: The name override will be converted to a lower snake case string
    name: Option<String>,

    ty: syn::Type,
    ident: Option<Ident>,
    attrs: Vec<syn::Attribute>,
}

#[derive(FromDeriveInput)]
#[darling(
    supports(enum_any, struct_any),
    attributes(mavlink),
    forward_attrs(doc, unit)
)]
pub struct MavlinkFieldSet {
    /// Optional name override for the enum. If let empty, the upper snake case
    /// enum name will be used as the default name.
    ///
    /// **Note**: The name override will be converted to a upper snake case string \
    /// **Note**: This attribute is only used for enum derives
    name: Option<String>,

    ident: Ident,
    data: darling::ast::Data<syn::Variant, MavlinkFieldSetField>,
    attrs: Vec<syn::Attribute>,
}

pub fn handle(fieldset: MavlinkFieldSet) -> TokenStream {
    if fieldset.data.is_enum() {
        handle_enum(fieldset)
    } else {
        let ident = fieldset.ident;
        let fieldset_fields = fieldset.data.take_struct().expect("should be a struct");
        if fieldset_fields.is_newtype() {
            handle_newtype_struct(&ident, &fieldset.attrs, &fieldset_fields)
        } else {
            handle_struct(&ident, &fieldset_fields)
        }
    }
}

pub fn handle_enum(fieldset: MavlinkFieldSet) -> TokenStream {
    let ident = fieldset.ident;
    let description_lit = LitStr::new(&fieldset.attrs.get_docs_str(), Span::call_site());
    let name = snake_case(&fieldset.name.clone().unwrap_or(ident.to_string())).to_uppercase();
    let name_lit = LitStr::new(&name, Span::call_site());
    let unit = match fieldset.attrs.get_unit() {
        Ok(Some(unit)) => quote!(Some(#unit)),
        Ok(None) => quote!(None),
        Err(err) => return err.write_errors(),
    };

    quote! {
        impl ::data_core::mavlink::hal::description::DescribeFieldSet for #ident {
            const FIELDS: &[::data_core::mavlink::hal::description::MessageField] =
                &[::data_core::mavlink::hal::description::MessageField {
                    type_: <#ident as ::data_core::mavlink::hal::typing::Typeable>::TYPE,
                    name: #name_lit,
                    unit: #unit,
                    description: #description_lit,
                    enum_: <#ident as ::data_core::mavlink::hal::typing::Typeable>::ENUM,
                }];
        }
    }
}

pub fn handle_newtype_struct(
    ident: &Ident,
    attrs: &Vec<syn::Attribute>,
    fields: &Fields<MavlinkFieldSetField>,
) -> TokenStream {
    let field = fields
        .fields
        .first()
        .expect("should have exactly one field");

    let description_lit = LitStr::new(&attrs.get_docs_str(), Span::call_site());
    let name = snake_case(&field.name.clone().unwrap_or(ident.to_string()));
    let name_lit = LitStr::new(&name, Span::call_site());
    let ty = &field.ty;
    let unit = match attrs.get_unit() {
        Ok(Some(unit)) => quote!(Some(#unit)),
        Ok(None) => quote!(None),
        Err(err) => return err.write_errors(),
    };

    quote! {
        impl ::data_core::mavlink::hal::description::DescribeFieldSet for #ident {
            const FIELDS: &[::data_core::mavlink::hal::description::MessageField] = &[
              ::data_core::mavlink::hal::description::MessageField {
                  type_: <#ty as ::data_core::mavlink::hal::typing::Typeable>::TYPE,
                  name: #name_lit,
                  unit: #unit,
                  description: #description_lit,
                  enum_: <#ty as ::data_core::mavlink::hal::typing::Typeable>::ENUM,
              }
            ];
        }
    }
}

pub fn handle_struct(ident: &Ident, fields: &Fields<MavlinkFieldSetField>) -> TokenStream {
    let mut error = darling::Error::accumulator();
    let mut flattened_sizes = Vec::new();
    let mut additional_fields = 0usize;

    let fields = fields.fields.iter().filter_map(|field| error.handle_in(|| {
           if field.flatten.is_present() && field.refine.is_present() {
               Err(darling::Error::custom("The `#[mavlink(flatten)]` and `#[mavlink(refine)]` attributes cannot be used at the same time!").with_span(&field.ident.span()))?;
           }

           let name = snake_case(&field.name.clone().unwrap_or(field.ident.as_ref().map(ToString::to_string).unwrap_or_default()));
           let name_lit = LitStr::new(&name, Span::call_site());
           let description_lit = LitStr::new(&field.attrs.get_docs_str(), Span::call_site());
           let ty = &field.ty;


           let result = if field.flatten.is_present() {
               flattened_sizes.push(quote! { <#ty as ::data_core::mavlink::hal::description::DescribeFieldSet>::FIELDS.len() });
               quote! { <#ty as ::data_core::mavlink::hal::description::DescribeFieldSet>::FIELDS }
           } else if field.refine.is_present() {
               additional_fields += 1;
               quote! {
                   &[::data_core::mavlink::hal::macro_util::refine_field(
                       <#ty as ::data_core::mavlink::hal::description::DescribeFieldSet>::FIELDS,
                       #name_lit,
                       #description_lit
                   )]
               }
           } else {
               let unit = match field.attrs.get_unit() {
                   Ok(Some(unit)) => quote!(Some(#unit)),
                   Ok(None) => quote!(None),
                   Err(err) => return Ok(err.write_errors()),
               };

               additional_fields += 1;
               quote! {
                   &[::data_core::mavlink::hal::description::MessageField {
                       type_: <#ty as ::data_core::mavlink::hal::typing::Typeable>::TYPE,
                       name: #name_lit,
                       unit: #unit,
                       description: #description_lit,
                       enum_: <#ty as ::data_core::mavlink::hal::typing::Typeable>::ENUM,
                   }]
               }
           };

           Ok(result)
        })).collect::<Vec<_>>();

    if let Err(err) = error.finish() {
        return err.write_errors();
    }

    let total_size = if flattened_sizes.is_empty() {
        quote!(#additional_fields)
    } else {
        quote! {
            #(#flattened_sizes)+* + #additional_fields
        }
    };

    quote! {
        impl ::data_core::mavlink::hal::description::DescribeFieldSet for #ident {
            const FIELDS: &[::data_core::mavlink::hal::description::MessageField] =
                &::data_core::mavlink::hal::macro_util::concat_slices::<_, { #total_size }>(&[
                    #(#fields),*
                ]);
        }
    }
}
