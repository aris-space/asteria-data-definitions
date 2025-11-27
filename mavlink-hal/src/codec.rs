pub use tbytes;

use tbytes::errors::TBytesError;
use tbytes::{TBytesReader, TBytesReaderBackend, TBytesWriter, TBytesWriterBackend};

use crate::macro_util::{total_size, total_size_by_align};
use crate::traits::DescribeFieldSet;

/// Implementors must ensure this matches the message fields they provide.
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

#[inline]
pub fn encode_all<T: Encode>(
    value: &T,
    writer: &mut TBytesWriter<impl TBytesWriterBackend>,
) -> Result<usize, TBytesError> {
    let mut written = 0;
    written += value.encode_8_aligned(writer)?;
    written += value.encode_4_aligned(writer)?;
    written += value.encode_2_aligned(writer)?;
    written += value.encode_1_aligned(writer)?;
    debug_assert_eq!(
        written,
        total_size(T::FIELDS),
        "the implementation should always write exactly as many bytes as are defined by the type"
    );
    Ok(written)
}

/// A type that construct themselves from a mavlink message.
///
/// The data of each alignment can be read from different readers.
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
#[inline]
pub fn decode_all<T: Decode>(payload: &[u8]) -> Result<T, TBytesError> {
    let payload_size: usize = total_size(T::FIELDS);
    if payload.len() > payload_size {
        return Err(todo!("figure out error handling"));
    }
    let mut full_payload = [0u8; 64]; // FIXME: we need max 64 bytes for CAN compatiblity, but mavlink has more
    full_payload[0..payload.len()].copy_from_slice(payload);
    let rest = &full_payload[..payload_size];
    let (payload_8, rest) = rest.split_at(total_size_by_align(T::FIELDS, 8));
    let (payload_4, rest) = rest.split_at(total_size_by_align(T::FIELDS, 4));
    let (payload_2, payload_1) = rest.split_at(total_size_by_align(T::FIELDS, 2));
    let mut reader_8 = TBytesReader::from(payload_8);
    let mut reader_4 = TBytesReader::from(payload_4);
    let mut reader_2 = TBytesReader::from(payload_2);
    let mut reader_1 = TBytesReader::from(payload_1);

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
