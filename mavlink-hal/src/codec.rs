pub use tbytes;

use tbytes::errors::TBytesError;
use tbytes::{TBytesWriter, TBytesWriterBackend};

use crate::traits::DescribeFieldSet;

/// Implementors must ensure this aligns with the message fields they provide.
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
    Ok(written)
}
