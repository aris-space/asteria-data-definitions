use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

/// Derives the `CanMessage` trait for a struct and assigns it a CAN ID using
/// the `#[can_id(...)]` attribute.
///
/// # Usage
/// ```ignore
/// #[derive(CanMessage)]
/// #[can_id(42)]
/// struct MyMessage { ... }
/// ```
///
/// The `#[can_id(...)]` attribute is **required** and must contain a single integer
/// literal representing the raw CAN ID. This integer may be in any format that Rust
/// supports, e.g. decimal or hexadecimal.
///
/// This macro generates:
/// ```ignore
/// impl CanMessage for MyMessage {
///     const ID_RAW: u16 = 42;
/// }
/// ```
#[proc_macro_derive(CanMessage, attributes(can_id))]
pub fn derive_can_message(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, attrs, .. } = parse_macro_input!(input);

    let mut can_id = None;
    for attr in &attrs {
        if attr.path().is_ident("can_id") {
            let lit_int: syn::LitInt = attr.parse_args().expect(
                "Error parsing #[can_id(...)]. \
                     Make sure you use parentheses and a single integer, e.g. #[can_id(10)].",
            );
            can_id = Some(lit_int);
        }
    }

    let can_id = can_id.expect(
        "Missing #[can_id(...)] attribute. \
         You must specify something like #[can_id(3)], using parentheses.",
    );

    let expanded = quote! {
        impl crate::CanMessage for #ident {
            const ID_RAW: u16 = #can_id;
        }
    };

    TokenStream::from(expanded)
}

/// Attribute macro that annotates a CAN message struct with common derives and a CAN ID to
/// reduce the boilerplate required for defining CAN messages.
///
/// This macro:
/// - Adds standard derives like `Clone`, `Debug`, `PartialEq`, `bincode::Encode`, `bincode::Decode`
/// - Applies optional derives conditionally (e.g. `serde`, `defmt`, `arbitrary`)
/// - Applies the required `#[can_id(...)]` attribute
/// - Automatically derives `CanMessage` using the ID
///
/// # Usage
/// ```ignore
/// #[can_message(7)]
/// pub struct Foo {
///     ...
/// }
/// ```
///
/// This expands to:
/// ```ignore
/// #[derive(Clone, Debug, PartialEq, ::bincode::Encode, ::bincode::Decode, ::hermes_can_derive::CanMessage)]
/// #[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
/// #[cfg_attr(feature = "defmt", derive(defmt::Format))]
/// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// #[can_id(7)]
/// pub struct Foo { ... }
/// ```
#[proc_macro_attribute]
pub fn can_message(attr: TokenStream, item: TokenStream) -> TokenStream {
    let can_id: syn::LitInt = syn::parse(attr).expect("expected integer literal");

    let item_ast: DeriveInput = syn::parse(item.clone()).unwrap();

    let expanded = quote! {
        #[derive(Clone, Debug, PartialEq, ::bincode::Encode, ::bincode::Decode, ::hermes_can_derive::CanMessage)]
        #[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[can_id(#can_id)]
        #item_ast
    };

    expanded.into()
}
