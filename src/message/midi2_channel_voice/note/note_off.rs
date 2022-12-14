use super::note_message;
note_message!(0b1000);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::midi2_channel_voice::attribute;
    use crate::{error::Error, util::message_traits_test};

    message_traits_test!(Message);

    #[test]
    fn wrong_type() {
        assert_eq!(
            Message::try_from_ump(&[0x1000_0000, 0x0]),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn wrong_status() {
        assert_eq!(
            Message::try_from_ump(&[0x4040_0000, 0x0]),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            Message::try_from_ump(&[0x408A_6301, 0xABCD_1234,]),
            Ok(Message {
                group: ux::u4::new(0x0),
                channel: ux::u4::new(0xA),
                note: ux::u7::new(0x63),
                velocity: 0xABCD,
                attribute: Some(attribute::Attribute::ManufacturerSpecific(0x1234)),
            })
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Message {
                group: ux::u4::new(0x5),
                channel: ux::u4::new(0x2),
                note: ux::u7::new(0x7B),
                velocity: 0x07A0,
                attribute: None
            }
            .to_ump(&mut [0x0, 0x0]),
            &[0x4582_7B00, 0x07A0_0000],
        );
    }
}
