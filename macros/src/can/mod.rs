use darling::ast::Fields;
use syn::Ident;

pub mod decode;
pub mod encode;
pub mod message;
pub mod sparse;

/// Tuple containing the identifier and fields of a CAN message variant
pub type CanVariantTuple<'a> = (&'a Ident, &'a Fields<syn::Field>);

/// Tuple containing the identifier, the path to the constant CAN id, the path to the variant type
/// and fields of a CAN message variant
pub type CanVariantTupleWithIdAndType<'a> = (
    &'a Ident,
    &'a syn::Path,
    &'a syn::Path,
    &'a Fields<syn::Field>,
);
