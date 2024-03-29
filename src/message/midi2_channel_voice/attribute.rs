use crate::{
    buffer::{Buffer, Ump},
    util::{
        schema::{Property, UmpSchema},
        BitOps, Truncate,
    },
    *,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Attribute {
    ManufacturerSpecific(u16),
    ProfileSpecific(u16),
    Pitch7_9 { note: u7, pitch_up: u9 },
}

pub fn validate_ump(bytes: &[u32]) -> Result<()> {
    match bytes[0].octet(3) {
        0x0 => Ok(()),
        0x1 => Ok(()),
        0x2 => Ok(()),
        0x3 => Ok(()),
        _ => Err(Error::InvalidData),
    }
}

pub fn from_ump(bytes: &[u32]) -> Option<Attribute> {
    match bytes[0].octet(3) {
        0x0 => None,
        0x1 => Some(Attribute::ManufacturerSpecific(bytes[1].word(1))),
        0x2 => Some(Attribute::ProfileSpecific(bytes[1].word(1))),
        0x3 => Some(Attribute::Pitch7_9 {
            note: (bytes[1].word(1) >> 9).truncate(),
            pitch_up: (bytes[1].word(1)).truncate(),
        }),
        _ => panic!("Invalid status"),
    }
}

pub fn write_attribute(bytes: &mut [u32], attr: Option<Attribute>) -> &mut [u32] {
    match attr {
        None => {
            bytes[0].set_octet(3, 0x0);
        }
        Some(a) => match a {
            Attribute::ManufacturerSpecific(d) => {
                bytes[0].set_octet(3, 0x1);
                bytes[1].set_word(1, d);
            }
            Attribute::ProfileSpecific(d) => {
                bytes[0].set_octet(3, 0x2);
                bytes[1].set_word(1, d);
            }
            Attribute::Pitch7_9 { note, pitch_up } => {
                let d = (u16::from(note) << 9) | u16::from(pitch_up);
                bytes[0].set_octet(3, 0x3);
                bytes[1].set_word(1, d);
            }
        },
    }
    &mut bytes[..2]
}

impl Property<Option<Attribute>, UmpSchema<0x0000_00FF, 0x0000_FFFF, 0x0, 0x0>, ()> for Ump {
    fn get(data: &[<Ump as Buffer>::Data]) -> Option<Attribute> {
        from_ump(data)
    }
    fn write(data: &mut [<Ump as Buffer>::Data], v: Option<Attribute>) {
        write_attribute(data, v);
    }
    fn validate(data: &[<Ump as Buffer>::Data]) -> Result<()> {
        validate_ump(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn try_from_ump(bytes: &[u32]) -> Result<Option<Attribute>> {
        validate_ump(bytes)?;
        Ok(from_ump(bytes))
    }

    #[test]
    fn from_packet_invalid() {
        assert_eq!(try_from_ump(&[0x0000_0004]), Err(Error::InvalidData),);
    }

    #[test]
    fn from_packet_none() {
        assert_eq!(try_from_ump(&[0x0, 0x0]), Ok(None),);
    }

    #[test]
    fn from_packet_generic() {
        assert_eq!(
            try_from_ump(&[0x0000_0001, 0x0000_1234]),
            Ok(Some(Attribute::ManufacturerSpecific(0x1234))),
        );
    }

    #[test]
    fn from_packet_pitch7_9() {
        assert_eq!(
            try_from_ump(&[0x0000_0003, 0b0000_0000_0000_0000_0011_0011_0011_0011]),
            Ok(Some(Attribute::Pitch7_9 {
                note: u7::new(0b0011001),
                pitch_up: u9::new(0b100110011)
            })),
        );
    }

    #[test]
    fn write_attribute_none() {
        assert_eq!(write_attribute(&mut [0x0, 0x0], None), &[0x0, 0x0],);
    }

    #[test]
    fn write_attribute_generic() {
        assert_eq!(
            write_attribute(&mut [0x0, 0x0], Some(Attribute::ProfileSpecific(0x0666))),
            &[0x0000_0002, 0x0000_0666],
        );
    }

    #[test]
    fn write_attribute_pitch7_9() {
        let attribute = Attribute::Pitch7_9 {
            note: u7::new(0b1011100),
            pitch_up: u9::new(0b100010111),
        };
        assert_eq!(
            write_attribute(&mut [0x0, 0x0], Some(attribute)),
            &[0x0000_0003, 0b0000_0000_0000_0000_1011_1001_0001_0111]
        );
    }
}
