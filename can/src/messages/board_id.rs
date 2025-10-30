/// The ID of a board in the rocket.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum BoardId {
    FlightComputer = 0,
    RfsComputer = 1,
    SensorCarrier = 2,
    CommunicationBoard = 3,
    EngineControlBoard = 4,
    PowerBoard = 5,
    RecoveryBoard = 6,
    FuelControlBoard = 7,
    OxidizerControlBoard = 8,
}

impl ::bincode::Encode for BoardId {
    fn encode<__E: ::bincode::enc::Encoder>(
        &self,
        encoder: &mut __E,
    ) -> Result<(), ::bincode::error::EncodeError> {
        <u8 as ::bincode::Encode>::encode(&(*self as u8), encoder)?;
        Ok(())
    }
}

impl<__Context> ::bincode::Decode<__Context> for BoardId {
    fn decode<__D: ::bincode::de::Decoder<Context = __Context>>(
        decoder: &mut __D,
    ) -> Result<Self, ::bincode::error::DecodeError> {
        let variant_index = <u8 as ::bincode::Decode<__D::Context>>::decode(decoder)?;
        match variant_index {
            0u8 => Ok(Self::FlightComputer {}),
            1u8 => Ok(Self::RfsComputer {}),
            2u8 => Ok(Self::SensorCarrier {}),
            3u8 => Ok(Self::CommunicationBoard {}),
            4u8 => Ok(Self::EngineControlBoard {}),
            5u8 => Ok(Self::PowerBoard {}),
            6u8 => Ok(Self::RecoveryBoard {}),
            7u8 => Ok(Self::FuelControlBoard {}),
            8u8 => Ok(Self::OxidizerControlBoard {}),
            variant => Err(::bincode::error::DecodeError::UnexpectedVariant {
                found: u32::from(variant),
                type_name: "BoardId",
                allowed: &::bincode::error::AllowedEnumVariants::Allowed(&[
                    0u32, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32,
                ]),
            }),
        }
    }
}
impl<'__de, __Context> ::bincode::BorrowDecode<'__de, __Context> for BoardId {
    fn borrow_decode<__D: ::bincode::de::BorrowDecoder<'__de, Context = __Context>>(
        decoder: &mut __D,
    ) -> Result<Self, ::bincode::error::DecodeError> {
        let variant_index = <u8 as ::bincode::Decode<__D::Context>>::decode(decoder)?;
        match variant_index {
            0u8 => Ok(Self::FlightComputer {}),
            1u8 => Ok(Self::RfsComputer {}),
            2u8 => Ok(Self::SensorCarrier {}),
            3u8 => Ok(Self::CommunicationBoard {}),
            4u8 => Ok(Self::EngineControlBoard {}),
            5u8 => Ok(Self::PowerBoard {}),
            6u8 => Ok(Self::RecoveryBoard {}),
            7u8 => Ok(Self::FuelControlBoard {}),
            8u8 => Ok(Self::OxidizerControlBoard {}),
            variant => Err(::bincode::error::DecodeError::UnexpectedVariant {
                found: u32::from(variant),
                type_name: "BoardId",
                allowed: &::bincode::error::AllowedEnumVariants::Allowed(&[
                    0u32, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32,
                ]),
            }),
        }
    }
}
