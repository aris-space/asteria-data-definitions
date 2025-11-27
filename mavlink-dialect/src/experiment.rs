#![allow(clippy::unwrap_used)]

use mavspec::rust::derive::Message;
// use mavspec::rust::derive::{Dialect, Enum, Message};

#[repr(u8)]
#[derive(Copy, Clone, Debug, Default)]
// #[derive(Copy, Clone, Debug, Default, Enum)]
enum Mood {
    #[default]
    Serious = 0,
    Grumpy = 1,
    Delighted = 2,
    Confused = 3,
}

// generated
impl core::convert::TryFrom<u8> for Mood {
    type Error = mavspec::rust::spec::SpecError;
    fn try_from(value: u8) -> core::result::Result<Self, mavspec::rust::spec::SpecError> {
        Ok(match value {
            0 => Self::Serious,
            1 => Self::Grumpy,
            2 => Self::Delighted,
            3 => Self::Confused,
            _ => {
                return Err(mavspec::rust::spec::SpecError::InvalidEnumValue { enum_name: "MOOD" });
            }
        })
    }
}

#[derive(Clone, Debug, Message)]
#[message_id(42)]
struct Howdy {
    #[base_type(u8)]
    mood: Mood,
}

#[derive(Clone, Debug)]
// #[derive(Clone, Debug, Message)]
// #[message_id(43)]
struct FineAndYou {
    // #[base_type(u8)]
    mood: Mood,
}

// generated:
// unnecessary I think
impl FineAndYou {
    #[doc = r" MavLink message ID."]
    pub const ID: mavspec::rust::spec::types::MessageId = 43;
    #[doc = r" Returns specification for this message."]
    #[inline]
    pub const fn spec() -> mavspec::rust::spec::MessageInfo {
        mavspec::rust::spec::MessageInfo::new(43, 195)
    }
    #[doc = r" Message `ID`."]
    #[inline]
    pub const fn message_id() -> mavspec::rust::spec::types::MessageId {
        43
    }
    #[doc = r" Message `CRC_EXTRA`."]
    #[inline]
    pub const fn crc_extra() -> mavspec::rust::spec::types::CrcExtra {
        195
    }
    #[doc = r" Minimum supported MAVLink version for this message."]
    #[inline]
    pub const fn min_supported_mavlink_version() -> mavspec::rust::spec::MavLinkVersion {
        mavspec::rust::spec::MavLinkVersion::V1
    }
}
// hm, is this needed? If so, for every variant, thus also a type for every variant...
impl mavspec::rust::spec::MessageSpecStatic for FineAndYou {
    #[inline]
    fn spec() -> mavspec::rust::spec::MessageInfo {
        FineAndYou::spec()
    }
    #[inline]
    fn message_id() -> mavspec::rust::spec::types::MessageId {
        43
    }
    #[inline]
    fn crc_extra() -> mavspec::rust::spec::types::CrcExtra {
        195
    }
    #[inline]
    fn min_supported_mavlink_version() -> mavspec::rust::spec::MavLinkVersion {
        mavspec::rust::spec::MavLinkVersion::V1
    }
}
// only necessary for enum I think
impl mavspec::rust::spec::MessageSpec for FineAndYou {
    #[inline]
    fn id(&self) -> mavspec::rust::spec::types::MessageId {
        43
    }
    #[inline]
    fn min_supported_mavlink_version(&self) -> mavspec::rust::spec::MavLinkVersion {
        mavspec::rust::spec::MavLinkVersion::V1
    }
    #[inline]
    fn crc_extra(&self) -> mavspec::rust::spec::types::CrcExtra {
        195
    }
}
// decode: can dispatch on ID.
// Aaaha, they use struct initialization patterns. Maybe this could work for us as well, despite 4x split?
impl TryFrom<&mavspec::rust::spec::Payload> for FineAndYou {
    type Error = mavspec::rust::spec::SpecError;
    fn try_from(value: &mavspec::rust::spec::Payload) -> core::result::Result<Self, Self::Error> {
        use mavspec::rust::spec::tbytes::{TBytesReader, TBytesReaderFor};
        #[inline]
        fn decode_v1(
            payload: &[u8],
        ) -> core::result::Result<FineAndYou, mavspec::rust::spec::SpecError> {
            const PAYLOAD_SIZE: usize = 0 + 1;
            if payload.len() != PAYLOAD_SIZE {
                return Err(mavspec::rust::spec::SpecError::InvalidV1PayloadSize {
                    actual: payload.len(),
                    expected: PAYLOAD_SIZE,
                });
            }
            let reader = TBytesReader::from(payload);
            Ok(FineAndYou {
                mood: {
                    let raw_value: u8 = reader.read().unwrap();
                    Mood::try_from(raw_value as u8)?
                },
            })
        }
        #[inline]
        fn decode_v2(
            payload: &[u8],
        ) -> core::result::Result<FineAndYou, mavspec::rust::spec::SpecError> {
            const PAYLOAD_SIZE: usize = 0 + 1;
            if payload.len() > PAYLOAD_SIZE {
                return Err(mavspec::rust::spec::SpecError::V2PayloadIsTooLong {
                    actual: payload.len(),
                    maximum: PAYLOAD_SIZE,
                });
            }
            let mut full_payload = [0u8; PAYLOAD_SIZE];
            full_payload[0..payload.len()].copy_from_slice(payload);
            let reader = TBytesReader::from(full_payload.as_slice());
            Ok(FineAndYou {
                mood: {
                    let raw_value: u8 = reader.read().unwrap();
                    Mood::try_from(raw_value as u8)?
                },
            })
        }
        match value.version() {
            mavspec::rust::spec::MavLinkVersion::V1 => decode_v1(value.bytes()),
            mavspec::rust::spec::MavLinkVersion::V2 => decode_v2(value.bytes()),
        }
    }
}
// encode: on enum, dispatch on variant
impl mavspec::rust::spec::IntoPayload for FineAndYou {
    fn encode(
        &self,
        version: mavspec::rust::spec::MavLinkVersion,
    ) -> core::result::Result<mavspec::rust::spec::Payload, mavspec::rust::spec::SpecError> {
        use mavspec::rust::spec::tbytes::{TBytesWriter, TBytesWriterFor};
        #[inline]
        fn encode_v1(
            message: &FineAndYou,
        ) -> core::result::Result<mavspec::rust::spec::Payload, mavspec::rust::spec::SpecError>
        {
            const PAYLOAD_SIZE: usize = 0 + 1;
            let mut buf = [0u8; PAYLOAD_SIZE];
            let mut writer = TBytesWriter::from(buf.as_mut_slice());
            writer
                .write({
                    let value = message.mood;
                    (value as u8) as u8
                })
                .unwrap();
            let payload = mavspec::rust::spec::Payload::new(
                43,
                buf.as_slice(),
                mavspec::rust::spec::MavLinkVersion::V1,
            );
            Ok(payload)
        }
        #[inline]
        fn encode_v2(
            message: &FineAndYou,
        ) -> core::result::Result<mavspec::rust::spec::Payload, mavspec::rust::spec::SpecError>
        {
            const PAYLOAD_SIZE: usize = 0 + 1;
            let mut buf = [0u8; PAYLOAD_SIZE];
            let mut writer = TBytesWriter::from(buf.as_mut_slice());
            writer
                .write({
                    let value = message.mood;
                    (value as u8) as u8
                })
                .unwrap();
            let payload = mavspec::rust::spec::Payload::new(
                43,
                buf.as_slice(),
                mavspec::rust::spec::MavLinkVersion::V2,
            );
            Ok(payload)
        }
        match version {
            mavspec::rust::spec::MavLinkVersion::V1 => encode_v1(&self),
            mavspec::rust::spec::MavLinkVersion::V2 => encode_v2(&self),
        }
    }
}
// not quite sure why, but should be easy for enum
impl mavspec::rust::spec::Message for FineAndYou {}
// dafuq: whyy is this derived?
impl core::default::Default for FineAndYou {
    fn default() -> Self {
        Self {
            mood: core::default::Default::default(),
        }
    }
}

// dialect would be in the topest level user.

#[derive(Clone, Debug)]
// #[derive(Clone, Debug, Dialect)]
// #[dialect(1099)]
// #[version(99)]
enum SmallTalk {
    Howdy(Howdy),
    FineAndYou(FineAndYou),
}

// generated:
// needs to be concatenated probably, needed for Dialect thing
const __MAVSPEC__MESSAGES__SMALL_TALK: [mavspec::rust::spec::MessageInfo; 2usize] = [
    mavspec::rust::spec::MessageInfo::new(Howdy::message_id(), Howdy::crc_extra()),
    mavspec::rust::spec::MessageInfo::new(FineAndYou::message_id(), FineAndYou::crc_extra()),
];
const __MAVSPEC__DIALECT_SPEC__SMALL_TALK: mavspec::rust::spec::DialectSpec =
    mavspec::rust::spec::DialectSpec::new(
        "small_talk",
        core::option::Option::Some(1099u32),
        core::option::Option::Some(99u8),
        &__MAVSPEC__MESSAGES__SMALL_TALK,
    );
// needs a messageInfo function for every data owner
// dispatched decode
impl mavspec::rust::spec::Dialect for SmallTalk {
    #[inline]
    fn name() -> &'static str {
        "small_talk"
    }
    #[inline]
    fn dialect() -> core::option::Option<mavspec::rust::spec::types::DialectId> {
        core::option::Option::Some(1099u32)
    }
    #[inline]
    fn version() -> core::option::Option<mavspec::rust::spec::types::DialectVersion> {
        core::option::Option::Some(99u8)
    }
    fn message_info(
        id: mavspec::rust::spec::types::MessageId,
    ) -> core::result::Result<
        &'static dyn mavspec::rust::spec::MessageSpec,
        mavspec::rust::spec::SpecError,
    > {
        for info in &__MAVSPEC__MESSAGES__SMALL_TALK {
            if info.id() == id {
                return Ok(info);
            }
        }
        Err(mavspec::rust::spec::SpecError::NotInDialect(id))
    }
    fn decode(
        payload: &mavspec::rust::spec::Payload,
    ) -> core::result::Result<Self, mavspec::rust::spec::SpecError> {
        #[allow(unreachable_patterns)]
        #[allow(unreachable_code)]
        Ok(match payload.id() {
            id if Howdy::message_id() == id => SmallTalk::Howdy(Howdy::try_from(payload)?),
            id if FineAndYou::message_id() == id => {
                SmallTalk::FineAndYou(FineAndYou::try_from(payload)?)
            }
            id => return Err(mavspec::rust::spec::SpecError::NotInDialect(id)),
        })
    }
    #[inline(always)]
    fn spec() -> &'static mavspec::rust::spec::DialectSpec {
        &__MAVSPEC__DIALECT_SPEC__SMALL_TALK
    }
}
// so boilerplatey, but okay
impl core::convert::TryFrom<&mavspec::rust::spec::Payload> for SmallTalk {
    type Error = mavspec::rust::spec::SpecError;
    fn try_from(value: &mavspec::rust::spec::Payload) -> core::result::Result<Self, Self::Error> {
        use mavspec::rust::spec::Dialect;
        Self::decode(value)
    }
}
// need to dispatch, but handle cases where the not a mavlink variant
impl mavspec::rust::spec::IntoPayload for SmallTalk {
    fn encode(
        &self,
        version: mavspec::rust::spec::MavLinkVersion,
    ) -> core::result::Result<mavspec::rust::spec::Payload, mavspec::rust::spec::SpecError> {
        #[allow(unreachable_patterns)]
        #[allow(unreachable_code)]
        Ok(match self {
            SmallTalk::Howdy(message) => message.encode(version)?,
            SmallTalk::FineAndYou(message) => message.encode(version)?,
            _ => unreachable!(),
        })
    }
}
// easy dispatch
impl mavspec::rust::spec::MessageSpec for SmallTalk {
    fn id(&self) -> mavspec::rust::spec::types::MessageId {
        #[allow(unreachable_patterns)]
        #[allow(unreachable_code)]
        match self {
            SmallTalk::Howdy(_) => Howdy::message_id(),
            SmallTalk::FineAndYou(_) => FineAndYou::message_id(),
            _ => unreachable!(),
        }
    }
    fn min_supported_mavlink_version(&self) -> mavspec::rust::spec::MavLinkVersion {
        #[allow(unreachable_patterns)]
        #[allow(unreachable_code)]
        match self {
            SmallTalk::Howdy(_) => Howdy::min_supported_mavlink_version(),
            SmallTalk::FineAndYou(_) => FineAndYou::min_supported_mavlink_version(),
            _ => unreachable!(),
        }
    }
    fn crc_extra(&self) -> mavspec::rust::spec::types::CrcExtra {
        #[allow(unreachable_patterns)]
        #[allow(unreachable_code)]
        match self {
            SmallTalk::Howdy(_) => Howdy::crc_extra(),
            SmallTalk::FineAndYou(_) => FineAndYou::crc_extra(),
            _ => unreachable!(),
        }
    }
}
