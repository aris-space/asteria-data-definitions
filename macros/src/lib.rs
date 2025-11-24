use darling::FromDeriveInput;
use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

use crate::can::CanMessage;

mod can;

#[proc_macro_derive(CanMessage, attributes(can))]
/// Derive macro for implementing the [`can_hal::Encode`] and [`can_hal::Decode`] traits for the message enum.
///
/// Additionally, it generates a module with type aliasses for all enum variants
/// and a module with a CAN id for every enum variant.
pub fn derive_can_message(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match CanMessage::from_derive_input(&input) {
        Ok(message) => can::handle(message),
        Err(err) => err.write_errors(),
    }
    .into()
}
