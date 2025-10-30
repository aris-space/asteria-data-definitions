/// Compile-time uniqueness check for CAN IDs.
#[macro_export]
macro_rules! check_unique_ids {
    ($($id:expr),* $(,)?) => {
        const _: () = {
            const IDS: &[u16] = &[$($id),*];
            let mut i = 0;
            while i < IDS.len() {
                let mut j = i + 1;
                while j < IDS.len() {
                    const_panic::concat_assert!( IDS[i] != IDS[j],
                         "Duplicate CAN ID detected: ",
                            IDS[i]
                     );
                    j += 1;
                }
                i += 1;
            }
        };
    };
}

/// Compile-time assertion for trait implementations.
#[macro_export]
macro_rules! assert_impl_all {
    ($trait:path, $( $(#[$attr:meta])* $ty:ty ),* $(,)?) => {
        const _: () = {
            const fn _assert_impl<T: $trait>() {}
            $(
                $(#[$attr])*
                _assert_impl::<$ty>();
            )*
        };
    };
}

/// Macro to define an enum with CAN messages. Defines an enum that wraps all
/// provided CAN message types.
///
/// # Parameters
/// - `$name`: Name of the generated enum (e.g., `Message`).
/// - `$variant`: Variant name for the enum.
/// - `$msg`: Concrete message type.
///
/// This macro also takes optional attributes for each message variant. These may
/// be used to conditionally include or exclude message variants based on features.
///
/// This macro additionally provides some convenience methods for the enum:
/// - `NUM_ENABLED_IDS`: Number of enabled CAN messages in this enum.
/// - `ENABLED_IDS`: CAN IDs of all enabled messages.
///   and implements the `From` trait for each message type to the enum variant.
///
/// # Example
/// ```no_run
/// use hermes_can::define_messages;
///
/// define_messages!(Message {
///     #[cfg(feature = "reset_all")]
///     ResetAll(messages::system_management::ResetAll),
///     #[cfg(feature = "reset_specific")]
///     ResetSpecific(messages::system_management::ResetSpecific),
/// });
///
/// for id in Message::ENABLED_IDS {
///     // ... enable filters for each ID
/// }
///
/// let msg: Message = messages::system_management::ResetAll.into();
/// let mut buf = [0u8; 64];
/// let (id, payload) = msg.try_into_parts(&mut buf).expect("Failed to encode message");
/// ```
#[macro_export]
macro_rules! define_messages {
    (
        $name:ident {
            $(
                $(#[$attr:meta])*
                $variant:ident($msg:ty)
            ),* $(,)?
        }
    ) => {
        /// Top-level enum that wraps all defined CAN message types.
        ///
        /// Each variant wraps a specific message type implementing the
        /// [`CanMessage`](crate::CanMessage) trait. This enum provides a common way to
        /// serialize and deserialize different message types over the CAN bus.
        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        #[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
        #[non_exhaustive]
        pub enum $name {
            $(
                $(#[$attr])*
                $variant($msg),
            )*
        }

        impl $name {
            /// Number of enabled CAN messages in this enum.
            pub const NUM_ENABLED_IDS: usize = {
                let mut count = 0;
                $(
                    $(#[$attr])*
                    {
                        let _ = <$msg as $crate::CanMessage>::ID_RAW;
                        count += 1;
                    }
                )*
                count
            };

            /// CAN IDs of all enabled messages. Includes all parseable IDs handled by this enum
            /// regardless of direction. To get only receiving IDs, filter out the transmit-only
            /// messages.
            pub const ENABLED_IDS: [::embedded_can::StandardId; Self::NUM_ENABLED_IDS] = [
                $(
                    $(#[$attr])*
                    <$msg as $crate::CanMessage>::ID,
                )*
            ];

            /// Decode the enum from a CAN message given by a CAN ID and payload buffer.
            ///
            /// The provided CAN ID is matched against the message types and then deserialized
            /// using its [`CanMessage::try_from_parts`](crate::CanMessage::try_from_parts) implementation.
            ///
            /// This method verifies that the provided CAN ID matches the expected value and attempts to decode
            /// a message from the data slice. It assumes that the slice contains only the message payload and
            /// that decoding can proceed without issue.
            ///
            /// This can be problematic if padding is present to satisfy the CAN FD data length code (DLC), but
            /// if the decoder consumes only the necessary bytes and ignores any trailing data (as `bincode` does),
            /// this should not interfere with deserialization.
            ///
            /// # Errors
            /// Returns [`CanDecodeError::UnknownId`](crate::CanDecodeError::UnknownId) if the ID is not recognized,
            /// or another variant of [`CanDecodeError`](crate::CanDecodeError) if decoding fails.
            pub fn try_from_parts(id: ::embedded_can::StandardId, data: &[u8]) -> Result<Self, $crate::CanDecodeError> {
                match id.as_raw() {
                    $(
                        $(#[$attr])*
                        <$msg as $crate::CanMessage>::ID_RAW => {
                            let msg = <$msg as $crate::CanMessage>::try_from_parts(id, data)?;
                            Ok(Self::$variant(msg))
                        }
                    )*
                    other => Err($crate::CanDecodeError::UnknownId(other)),
                }
            }

            /// Serialize the message associated with the variant into a buffer and return the CAN ID
            /// and the number of bytes written.
            ///
            /// This method is simply a convenience wrapper around the
            /// [`CanMessage::try_write_into`](crate::CanMessage::try_write_into) method.
            /// It encodes the message into the provided buffer and returns the number of bytes
            /// written, corresponding exactly to the serialized data without any padding.
            ///
            /// # Important
            /// - Some CAN frameworks require that the data length is representable by a valid FD-CAN
            ///   data length code (DLC).
            /// - This function does not verify or add padding to meet DLC requirements; it only ensures
            ///   that the encoded message does not exceed the maximum CAN FD length (64 bytes).
            /// - It is the caller’s responsibility to pad the output if necessary.
            ///
            /// # Errors
            /// Returns [`CanEncodeError::MessageTooLong`](crate::CanEncodeError::MessageTooLong) if the encoded
            /// message exceeds the CAN FD length, or another variant of
            /// [`CanEncodeError`](crate::CanEncodeError) if serialization fails.
            pub fn try_write_into(&self, buf: &mut [u8]) -> Result<(::embedded_can::StandardId, usize), $crate::CanEncodeError> {
                match self {
                    $(
                        $(#[$attr])*
                        $name::$variant(msg) => <$msg as $crate::CanMessage>::try_write_into(msg, buf),
                    )*
                }
            }

            /// Returns the CAN id of this message.
            #[must_use]
            pub fn id(&self) -> ::embedded_can::StandardId {
                match self {
                    $(
                        $(#[$attr])*
                        $name::$variant(_) => <$msg as $crate::CanMessage>::ID,
                    )*
                }
            }
        }

        $(
            // Convenience conversion from the message type to the enum variant.
            $(#[$attr])*
            impl From<$msg> for $name {
                fn from(value: $msg) -> Self {
                    $name::$variant(value)
                }
            }
        )*

        $crate::check_unique_ids!($(<$msg as $crate::CanMessage>::ID_RAW),*);
        $crate::assert_impl_all!($crate::CanMessage, $( $(#[$attr])* $msg ),*);
    };
}
