pub use tbytes;

use tbytes::errors::TBytesError;
use tbytes::{TBytesReader, TBytesReaderBackend, TBytesWriter, TBytesWriterBackend};

use crate::macro_util::{total_size, total_size_by_align};
use crate::description::DescribeFieldSet;

/// A type that can encode itself as part of a mavlink message.
///
/// The 4 functions are chosen such that no reordering (<https://mavlink.io/en/guide/serialization.html#field_reordering>)
/// needs to happen, but the message parts can be encoded recursively.
/// Implementors must ensure this matches the message fields they provide with [`DescribeFieldSet`].
pub trait Encode: DescribeFieldSet {
    fn encode_8_aligned(
        &self,
        writer: &mut TBytesWriter<impl TBytesWriterBackend>,
    ) -> Result<usize, TBytesError>;
    fn encode_4_aligned(
        &self,
        writer: &mut TBytesWriter<impl TBytesWriterBackend>,
    ) -> Result<usize, TBytesError>;
    fn encode_2_aligned(
        &self,
        writer: &mut TBytesWriter<impl TBytesWriterBackend>,
    ) -> Result<usize, TBytesError>;
    fn encode_1_aligned(
        &self,
        writer: &mut TBytesWriter<impl TBytesWriterBackend>,
    ) -> Result<usize, TBytesError>;
}

/// Encodes a full message body `value`, with payload size `N`.
///
/// For `N` you should use `total_size(T::FIELDS)`, but this can't be automated.
#[inline]
pub fn encode_body<T: Encode, const N: usize>(value: &T) -> Result<[u8; N], TBytesError> {
    debug_assert_eq!(N, total_size(T::FIELDS), "incompatible N chosen");
    let mut buffer = [0_u8; N];
    let mut writer = tbytes::TBytesWriter::from(buffer.as_mut_slice());
    let mut written = 0;
    written += value.encode_8_aligned(&mut writer)?;
    written += value.encode_4_aligned(&mut writer)?;
    written += value.encode_2_aligned(&mut writer)?;
    written += value.encode_1_aligned(&mut writer)?;
    debug_assert_eq!(
        written,
        total_size(T::FIELDS),
        "the implementation should always write exactly as many bytes as are defined by the type"
    );
    Ok(buffer)
}

/// A type that can construct itself from a mavlink message part.
///
/// The data of each alignment can be read from different readers,
/// such that no reordering (<https://mavlink.io/en/guide/serialization.html#field_reordering>) needs to happen.
pub trait Decode: DescribeFieldSet + Sized {
    // technically no `&mut` is required as the backend uses interiour mutability, which is sad.
    fn decode(
        reader_8_aligned: &mut TBytesReader<impl TBytesReaderBackend>,
        reader_4_aligned: &mut TBytesReader<impl TBytesReaderBackend>,
        reader_2_aligned: &mut TBytesReader<impl TBytesReaderBackend>,
        reader_1_aligned: &mut TBytesReader<impl TBytesReaderBackend>,
    ) -> Result<Self, TBytesError>;
}

/// Decode a full message. This must not be called recursively.
///
/// For `N` you should use `total_size(T::FIELDS)`, but this can't be automated.
#[inline]
pub fn decode_body<T: Decode, const N: usize>(payload: &[u8]) -> Result<T, TBytesError> {
    let payload_size: usize = total_size(T::FIELDS);
    debug_assert_eq!(N, payload_size, "incompatible N chosen");
    if payload.len() > payload_size {
        return Err(TBytesError::OutOfBounds); // the only error that exists.
    }
    // zero extend the payload
    let mut full_payload = [0u8; N];
    full_payload[0..payload.len()].copy_from_slice(payload);

    // split the payload by alignment
    let rest = full_payload;
    let (payload_8, rest) = rest.split_at(total_size_by_align(T::FIELDS, 8));
    let (payload_4, rest) = rest.split_at(total_size_by_align(T::FIELDS, 4));
    let (payload_2, payload_1) = rest.split_at(total_size_by_align(T::FIELDS, 2));
    let mut reader_8 = TBytesReader::from(payload_8);
    let mut reader_4 = TBytesReader::from(payload_4);
    let mut reader_2 = TBytesReader::from(payload_2);
    let mut reader_1 = TBytesReader::from(payload_1);

    // decode and then check it was fully decoded
    let res = T::decode(&mut reader_8, &mut reader_4, &mut reader_2, &mut reader_1);
    debug_assert!(
        tbytes::TBytesReaderFor::<u8>::read(&reader_8).is_err(),
        "must have read all bytes"
    );
    debug_assert!(
        tbytes::TBytesReaderFor::<u8>::read(&reader_4).is_err(),
        "must have read all bytes"
    );
    debug_assert!(
        tbytes::TBytesReaderFor::<u8>::read(&reader_2).is_err(),
        "must have read all bytes"
    );
    debug_assert!(
        tbytes::TBytesReaderFor::<u8>::read(&reader_1).is_err(),
        "must have read all bytes"
    );
    res
}
