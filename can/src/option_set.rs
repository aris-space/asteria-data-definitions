#[macro_export]
macro_rules! define_option_set {
    ($name:ident, $( $ty:ident : $field:ident ),+ $(,)?) => {
        /// A struct with N `Option<T>` fields, one per type parameter.
        ///
        /// Provides a compact `bincode` encoding: a bitfield for presence, followed by
        /// optional values if they are present.
        ///
        /// Derives common traits and optionally supports `arbitrary`, `defmt`, and `serde`.
        #[allow(non_snake_case)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
        #[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $name<$( $ty ),+> {
            $( pub $field: core::option::Option<$ty>, )+
        }

        impl<$($ty: ::bincode::Encode),+> ::bincode::Encode for $name<$( $ty ),+> {
            fn encode<Enc: ::bincode::enc::Encoder>(
                &self,
                encoder: &mut Enc
            ) -> Result<(), ::bincode::error::EncodeError> {
                const FIELD_COUNT: usize = 0_usize $(+ { let _ = stringify!($ty); 1 })+;
                const BITFIELD_BYTE_COUNT: usize = FIELD_COUNT.div_ceil(8);

                let mut bitfield = [0u8; BITFIELD_BYTE_COUNT];
                let mut idx = 0;
                $(
                    if self.$field.is_some() {
                        let byte_idx = idx / 8;
                        let bit_idx  = idx % 8;
                        bitfield[byte_idx] |= 1 << bit_idx;
                    }
                    #[allow(unused_assignments)]
                    {
                        idx += 1;
                    }
                )+
                ::bincode::Encode::encode(&bitfield, encoder)?;
                $(
                    if let Some(ref val) = self.$field {
                        val.encode(encoder)?;
                    }
                )+
                Ok(())
            }
        }

        impl<$($ty: ::bincode::Decode<Ctx>),+, Ctx> ::bincode::Decode<Ctx> for $name<$( $ty ),+> {
            fn decode<Dec: ::bincode::de::Decoder<Context = Ctx>>(
                decoder: &mut Dec
            ) -> Result<Self, ::bincode::error::DecodeError> {
                const FIELD_COUNT: usize = 0_usize $(+ { let _ = stringify!($ty); 1 })+;
                const BITFIELD_BYTE_COUNT: usize = FIELD_COUNT.div_ceil(8);
                let bitfield: [u8; BITFIELD_BYTE_COUNT] = ::bincode::de::Decode::decode(decoder)?;
                let mut idx = 0;
                Ok(Self {
                    $(
                        $field: {
                            let byte_idx = idx / 8;
                            let bit_idx  = idx % 8;
                            #[allow(unused_assignments)]
                            {
                                idx += 1;
                            }
                            if bitfield[byte_idx] & (1 << bit_idx) != 0 {
                                Some($ty::decode(decoder)?)
                            } else {
                                None
                            }
                        },
                    )+
                })
            }
        }

        impl<'de, $($ty: ::bincode::de::BorrowDecode<'de, Ctx>),+, Ctx> ::bincode::de::BorrowDecode<'de, Ctx> for $name<$( $ty ),+> {
            fn borrow_decode<Dec: ::bincode::de::BorrowDecoder<'de, Context = Ctx>>(
                decoder: &mut Dec
            ) -> Result<Self, ::bincode::error::DecodeError> {
                const FIELD_COUNT: usize = 0_usize $(+ { let _ = stringify!($ty); 1 })+;
                const BITFIELD_BYTE_COUNT: usize = FIELD_COUNT.div_ceil(8);
                let bitfield: [u8; BITFIELD_BYTE_COUNT] = ::bincode::de::BorrowDecode::borrow_decode(decoder)?;
                let mut idx = 0;
                Ok(Self {
                    $(
                        $field: {
                            let byte_idx = idx / 8;
                            let bit_idx  = idx % 8;
                            #[allow(unused_assignments)]
                            {
                                idx += 1;
                            }
                            if bitfield[byte_idx] & (1 << bit_idx) != 0 {
                                Some($ty::borrow_decode(decoder)?)
                            } else {
                                None
                            }
                        },
                    )+
                })
            }
        }
    }
}

define_option_set!(OptionSet2, A: a, B: b);
define_option_set!(OptionSet3, A: a, B: b, C: c);
define_option_set!(OptionSet4, A: a, B: b, C: c, D: d);
define_option_set!(OptionSet5, A: a, B: b, C: c, D: d, E: e);
define_option_set!(OptionSet6, A: a, B: b, C: c, D: d, E: e, F: f);
define_option_set!(OptionSet7, A: a, B: b, C: c, D: d, E: e, F: f, G: g);
define_option_set!(OptionSet8, A: a, B: b, C: c, D: d, E: e, F: f, G: g, H: h);
define_option_set!(OptionSet9, A: a, B: b, C: c, D: d, E: e, F: f, G: g, H: h, I: i);
define_option_set!(OptionSet10, A: a, B: b, C: c, D: d, E: e, F: f, G: g, H: h, I: i, J: j);

// Convenience type aliases for `OptionSetN` where all fields are of the same type.
pub type OptionSet2All<T> = OptionSet2<T, T>;
pub type OptionSet3All<T> = OptionSet3<T, T, T>;
pub type OptionSet4All<T> = OptionSet4<T, T, T, T>;
pub type OptionSet5All<T> = OptionSet5<T, T, T, T, T>;
pub type OptionSet6All<T> = OptionSet6<T, T, T, T, T, T>;
pub type OptionSet7All<T> = OptionSet7<T, T, T, T, T, T, T>;
pub type OptionSet8All<T> = OptionSet8<T, T, T, T, T, T, T, T>;
pub type OptionSet9All<T> = OptionSet9<T, T, T, T, T, T, T, T, T>;
pub type OptionSet10All<T> = OptionSet10<T, T, T, T, T, T, T, T, T, T>;

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::unreadable_literal)]
    extern crate std;
    use super::*;
    use bincode::config::{Fixint, Limit, LittleEndian, standard};
    use std::prelude::rust_2021::*;

    const BINCODE_CONFIG: bincode::config::Configuration<LittleEndian, Fixint, Limit<64>> =
        standard().with_fixed_int_encoding().with_limit();

    #[test]
    fn test_option_set2_encoding() {
        // All None: bitfield 0x00, length 1.
        let os = OptionSet2::<u32, u32> { a: None, b: None };
        let encoded = bincode::encode_to_vec(os, BINCODE_CONFIG).unwrap();
        assert_eq!(encoded.len(), 1);
        assert_eq!(encoded[0], 0x00);

        // Only a Some: bitfield 0x01, length 1+4.
        let os: OptionSet2All<i32> = OptionSet2 {
            a: Some(10),
            b: None,
        };
        let encoded = bincode::encode_to_vec(os, BINCODE_CONFIG).unwrap();
        assert_eq!(encoded.len(), 5);
        assert_eq!(encoded[0], 0x01);
        let mut expected = std::vec![0x01];
        bincode::encode_into_std_write(10u32, &mut expected, BINCODE_CONFIG).unwrap();
        assert_eq!(encoded, expected);

        // Only b Some: bitfield 0x02.
        let os: OptionSet2All<i32> = OptionSet2 {
            a: None,
            b: Some(20),
        };
        let encoded = bincode::encode_to_vec(os, BINCODE_CONFIG).unwrap();
        assert_eq!(encoded.len(), 5);
        assert_eq!(encoded[0], 0x02);

        // Both Some: bitfield 0x03, length 1+4+4.
        let os = OptionSet2 {
            a: Some(10),
            b: Some(20),
        };
        let encoded = bincode::encode_to_vec(os, BINCODE_CONFIG).unwrap();
        assert_eq!(encoded.len(), 9);
        assert_eq!(encoded[0], 0x03);
        let mut expected = std::vec![0x03];
        bincode::encode_into_std_write(10u32, &mut expected, BINCODE_CONFIG).unwrap();
        bincode::encode_into_std_write(20u32, &mut expected, BINCODE_CONFIG).unwrap();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_option_set10_encoding() {
        // All None: bitfield [0x00, 0x00], length 2.
        let os = OptionSet10All::<u8>::default();
        let encoded = bincode::encode_to_vec(os, BINCODE_CONFIG).unwrap();
        assert_eq!(encoded.len(), 2);
        assert_eq!(encoded[0], 0x00);
        assert_eq!(encoded[1], 0x00);

        // Only field i Some (index 8): second byte bit 0 set.
        let os = OptionSet10All::<u8> {
            a: None,
            b: None,
            c: None,
            d: None,
            e: None,
            f: None,
            g: None,
            h: None,
            i: Some(9),
            j: None,
        };
        let encoded = bincode::encode_to_vec(os, BINCODE_CONFIG).unwrap();
        assert_eq!(encoded.len(), 3);
        assert_eq!(encoded[0], 0x00);
        assert_eq!(encoded[1], 0x01);

        // Only field j Some (index 9): second byte bit 1 set.
        let os = OptionSet10All::<u8> {
            a: None,
            b: None,
            c: None,
            d: None,
            e: None,
            f: None,
            g: None,
            h: None,
            i: None,
            j: Some(10),
        };
        let encoded = bincode::encode_to_vec(os, BINCODE_CONFIG).unwrap();
        assert_eq!(encoded.len(), 3);
        assert_eq!(encoded[0], 0x00);
        assert_eq!(encoded[1], 0x02);

        // Fields a and j Some: first byte bit 0 and second byte bit 1 set.
        let os = OptionSet10All::<u8> {
            a: Some(1),
            b: None,
            c: None,
            d: None,
            e: None,
            f: None,
            g: None,
            h: None,
            i: None,
            j: Some(10),
        };
        let encoded = bincode::encode_to_vec(os, BINCODE_CONFIG).unwrap();
        assert_eq!(encoded.len(), 4);
        assert_eq!(encoded[0], 0x01);
        assert_eq!(encoded[1], 0x02);

        // Sparse encoding: fields a, c, i Some.
        let os = OptionSet10All::<u32> {
            a: Some(123),
            b: None,
            c: Some(456),
            d: None,
            e: None,
            f: None,
            g: None,
            h: None,
            i: Some(789),
            j: None,
        };
        let encoded = bincode::encode_to_vec(os, BINCODE_CONFIG).unwrap();
        let mut expected = std::vec![0b00000101, 0b00000001];
        bincode::encode_into_std_write(123u32, &mut expected, BINCODE_CONFIG).unwrap();
        bincode::encode_into_std_write(456u32, &mut expected, BINCODE_CONFIG).unwrap();
        bincode::encode_into_std_write(789u32, &mut expected, BINCODE_CONFIG).unwrap();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_option_set3_4_7_8_9_encoding() {
        // OptionSet3: a and c Some => bitfield 0x05, length 3.
        let os = OptionSet3All::<u8> {
            a: Some(1),
            b: None,
            c: Some(3),
        };
        let encoded = bincode::encode_to_vec(os, BINCODE_CONFIG).unwrap();
        assert_eq!(encoded.len(), 3);
        assert_eq!(encoded[0], 0x05);

        // OptionSet4: all Some => bitfield 0x0F, length 5.
        let os = OptionSet4All::<u8> {
            a: Some(1),
            b: Some(2),
            c: Some(3),
            d: Some(4),
        };
        let encoded = bincode::encode_to_vec(os, BINCODE_CONFIG).unwrap();
        assert_eq!(encoded.len(), 5);
        assert_eq!(encoded[0], 0x0F);

        // OptionSet7: fields b, d, f Some => bitfield 0b00101010.
        let os = OptionSet7All::<u8> {
            a: None,
            b: Some(2),
            c: None,
            d: Some(4),
            e: None,
            f: Some(6),
            g: None,
        };
        let encoded = bincode::encode_to_vec(os, BINCODE_CONFIG).unwrap();
        assert_eq!(encoded.len(), 4);
        assert_eq!(encoded[0], 0b00101010);

        // OptionSet8: all Some => bitfield 0xFF, length 9.
        let os = OptionSet8All::<u8> {
            a: Some(1),
            b: Some(2),
            c: Some(3),
            d: Some(4),
            e: Some(5),
            f: Some(6),
            g: Some(7),
            h: Some(8),
        };
        let encoded = bincode::encode_to_vec(os, BINCODE_CONFIG).unwrap();
        assert_eq!(encoded.len(), 9);
        assert_eq!(encoded[0], 0xFF);

        // OptionSet9: a and i Some => bitfield [0x01, 0x01], length 4.
        let os = OptionSet9All::<u8> {
            a: Some(1),
            b: None,
            c: None,
            d: None,
            e: None,
            f: None,
            g: None,
            h: None,
            i: Some(9),
        };
        let encoded = bincode::encode_to_vec(os, BINCODE_CONFIG).unwrap();
        assert_eq!(encoded.len(), 4);
        assert_eq!(encoded[0], 0x01);
        assert_eq!(encoded[1], 0x01);
    }

    #[test]
    fn test_option_set4_all_some_encoding() {
        let os = OptionSet4::<u16, u32, u8, u32> {
            a: Some(5),
            b: Some(1000),
            c: Some(8),
            d: Some(123456),
        };
        let encoded = bincode::encode_to_vec(os, BINCODE_CONFIG).unwrap();
        let mut expected = std::vec![0b1111];
        bincode::encode_into_std_write(5u16, &mut expected, BINCODE_CONFIG).unwrap();
        bincode::encode_into_std_write(1000u32, &mut expected, BINCODE_CONFIG).unwrap();
        expected.push(8);
        bincode::encode_into_std_write(123456u32, &mut expected, BINCODE_CONFIG).unwrap();
        assert_eq!(encoded, expected);
    }
}
