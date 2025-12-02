// Allow broken doc links in general for the macros as the types referenced
// in the rustdoc don't need to be available in this crate but only in the
// target crate
#![allow(rustdoc::broken_intra_doc_links)]
// The [`darling::FromVariant`] macro seems to have needless continues
#![allow(clippy::needless_continue)]

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

use message::Message;

#[cfg(feature = "can")]
mod can;
mod message;

/// Derive macro for generating a module with type aliasses for all enum variants
#[proc_macro_derive(Message)]
pub fn derive_message(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match Message::from_derive_input(&input) {
        Ok(message) => message::handle(message),
        Err(err) => err.write_errors(),
    }
    .into()
}

/// Derive macro for implementing the [`can_hal::Encode`] and [`can_hal::Decode`] traits for the message enum.
///
/// Additionally, it generates a module with a CAN id for every enum variant.
#[cfg(feature = "can")]
#[proc_macro_derive(CanMessage, attributes(can))]
pub fn derive_can_message(input: TokenStream) -> TokenStream {
    use crate::can::message::{CanMessage, handle};

    let input = parse_macro_input!(input as DeriveInput);
    match CanMessage::from_derive_input(&input) {
        Ok(message) => handle(message),
        Err(err) => err.write_errors(),
    }
    .into()
}

/// Derive macro for implementing the [`can_hal::Decode`] trait for the message enum.
///
/// The enum needs to be annotated with one of the two mutually exclusive attributes:
/// - `#[can(sparse)]`: Implement sparse decoding for the message enum. Sparse decoding should not be implemented using
///   this macro directly, instead use the [`sparse_decodable_can_message`] macro.
/// - `#[can(dispatch)]`: Implement dispatch decoding for the message enum. This means that every variant
///   of the enum needs to be a message enum itself.
///   **Important**: When using the dispatch decoding, it's required that you have the [thiserror](https://crates.io/crates/thiserror) crate
///   available in your current crate
#[cfg(feature = "can")]
#[proc_macro_derive(CanDecode, attributes(can))]
pub fn derive_can_decode(input: TokenStream) -> TokenStream {
    use crate::can::decode::{CanDecode, handle};

    let input = parse_macro_input!(input as DeriveInput);
    match CanDecode::from_derive_input(&input) {
        Ok(decode) => handle(decode),
        Err(err) => err.write_errors(),
    }
    .into()
}

/// Derive macro for implementing the [`can_hal::Encode`] trait for the message enum.
///
/// The enum needs to be annotated with one of the two mutually exclusive attributes:
/// - `#[can(sparse)]`: Implement sparse encoding for the message enum. Sparse encoding should not be implemented using
///   this macro directly, instead use the [`sparse_encodable_can_message`] macro.
/// - `#[can(dispatch)]`: Implement dispatch encoding for the message enum. This means that every variant
///   of the enum needs to be a message enum itself.
///   **Important**: When using the dispatch encoding, it's required that you have the [thiserror](https://crates.io/crates/thiserror) crate
///   available in your current crate
#[cfg(feature = "can")]
#[proc_macro_derive(CanEncode, attributes(can))]
pub fn derive_can_encode(input: TokenStream) -> TokenStream {
    use crate::can::encode::{CanEncode, handle};

    let input = parse_macro_input!(input as DeriveInput);
    match CanEncode::from_derive_input(&input) {
        Ok(encode) => handle(encode),
        Err(err) => err.write_errors(),
    }
    .into()
}

/// Create a CAN message enum which is sparsely decodable.
/// It automatically derives the [`CanDecode`] macro with the `#[can(sparse)]` attribute on the new enum.
///
/// This macro supports pseudo-rust syntax for defining a new enum. All variant types need to be
/// a path consisting of two segments:
/// - The first segment represents the **module** where the message enum for this variant is defined
/// - The second segment represents the **variant name** within the message enum for this variant
/// # Usage
/// ```rust
/// sparse_decodable_can_message! {
///     enum SparseMessage {
///         // here `ResetBoard` is a variant of `dp_power_board::Message::ResetBoard`
///         Reset(dp_power_board::ResetBoard)
///     }
/// }
/// ```
#[cfg(feature = "can")]
#[proc_macro]
pub fn sparse_decodable_can_message(input: TokenStream) -> TokenStream {
    use crate::can::sparse::{SparseCanMessage, handle_decode};

    let input = parse_macro_input!(input as SparseCanMessage);
    handle_decode(&input).into()
}

/// Create a CAN message enum which is sparsely encodable.
/// It automatically derives the [`CanEncode`] macro with the `#[can(sparse)]` attribute on the new enum.
///
/// This macro supports pseudo-rust syntax for defining a new enum. All variant types need to be
/// a path consisting of two segments:
/// - The first segment represents the **module** where the message enum for this variant is defined
/// - The second segment represents the **variant name** within the message enum for this variant
/// # Usage
/// ```rust
/// sparse_encodable_can_message! {
///     enum SparseMessage {
///         // here `ResetBoard` is a variant of `dp_power_board::Message::ResetBoard`
///         Reset(dp_power_board::ResetBoard)
///     }
/// }
/// ```
#[cfg(feature = "can")]
#[proc_macro]
pub fn sparse_encodable_can_message(input: TokenStream) -> TokenStream {
    use crate::can::sparse::{SparseCanMessage, handle_encode};

    let input = parse_macro_input!(input as SparseCanMessage);
    handle_encode(&input).into()
}
