// Allow broken doc links in general for the macros as the types referenced
// in the rustdoc don't need to be available in this crate but only in the
// target crate
#![allow(rustdoc::broken_intra_doc_links)]

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

use can::message::CanMessage;
use message::Message;

use crate::can::{decode::CanDecode, encode::CanEncode};

mod can;
mod message;

#[proc_macro_derive(Message, attributes(can))]
/// Derive macro for generating a module with type aliasses for all enum variants
pub fn derive_message(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match Message::from_derive_input(&input) {
        Ok(message) => message::handle(message),
        Err(err) => err.write_errors(),
    }
    .into()
}

#[proc_macro_derive(CanMessage, attributes(can))]
/// Derive macro for implementing the [`can_hal::Encode`] and [`can_hal::Decode`] traits for the message enum.
///
/// Additionally, it generates a module with a CAN id for every enum variant.
///
/// **Important**: Ensure that the [bincode](https://crates.io/crates/bincode) dependency is available
/// in the current crate should you be using this derive on enums with non-newtype
/// variants (struct variants or tuple variants with more than one field)
pub fn derive_can_message(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match CanMessage::from_derive_input(&input) {
        Ok(message) => can::message::handle(message),
        Err(err) => err.write_errors(),
    }
    .into()
}

#[proc_macro_derive(CanDecode, attributes(can))]
pub fn derive_can_decode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match CanDecode::from_derive_input(&input) {
        Ok(decode) => can::decode::handle(decode),
        Err(err) => err.write_errors(),
    }
    .into()
}

#[proc_macro_derive(CanEncode, attributes(can))]
pub fn derive_can_encode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match CanEncode::from_derive_input(&input) {
        Ok(encode) => can::encode::handle(encode),
        Err(err) => err.write_errors(),
    }
    .into()
}