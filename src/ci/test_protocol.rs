use crate::{
    ci::{helpers as ci_helpers, CiMessageDetail, DeviceId},
    error::Error,
    util::{builder, getter, sysex_message},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    group: ux::u4,
    source: ux::u28,
    destination: ux::u28,
    authority_level: ux::u7,
}

builder::builder!(
    group: ux::u4,
    source: ux::u28,
    destination: ux::u28,
    authority_level: ux::u7
);

impl Message {
    const STATUS: u8 = 0x13;
    const DATA_SIZE: usize = 62;
    getter::getter!(group, ux::u4);
    getter::getter!(source, ux::u28);
    getter::getter!(destination, ux::u28);
    getter::getter!(authority_level, ux::u7);
}

impl CiMessageDetail for Message {
    fn to_sysex<'a, M: sysex_message::SysexMessage>(&self, messages: &'a mut [M]) -> &'a mut [M] {
        let mut test_data_buffer = [ux::u7::default(); 48];
        ci_helpers::write_ci_data(
            self.group,
            DeviceId::MidiPort,
            Message::STATUS,
            self.source,
            self.destination,
            &[&[self.authority_level], test_data(&mut test_data_buffer)].concat(),
            messages,
        )
    }

    fn from_sysex<M: sysex_message::SysexMessage>(_messages: &[M]) -> Self {
        todo!()
    }

    fn validate_sysex<M: sysex_message::SysexMessage>(_messages: &[M]) -> Result<(), Error> {
        todo!()
    }

    fn validate_to_sysex_buffer<M: sysex_message::SysexMessage>(
        &self,
        messages: &[M],
    ) -> Result<(), Error> {
        ci_helpers::validate_buffer_size(messages, Message::DATA_SIZE)
    }
}

fn test_data(buff: &mut [ux::u7]) -> &[ux::u7] {
    for i in 0u8..48u8 {
        buff[i as usize] = ux::u7::new(i);
    }
    buff
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ci::{CiMessage, VERSION},
        message::system_exclusive_8bit as sysex8,
    };
    
    const TEST_STREAM_ID: u8 = 0x88;
    const TEST_GROUP: ux::u4 = ux::u4::new(0xE);
    const TEST_SOURCE: ux::u28 = ux::u28::new(31193279);
    const TEST_DESTINATION: ux::u28 = ux::u28::new(196547546);
    const TEST_AUTHORITY_LEVEL: ux::u7 = ux::u7::new(0x19);
    
    #[test]
    #[rustfmt::skip]
    fn try_to_sysex8() {
        assert_eq!(
            Message::builder()
                .group(TEST_GROUP)
                .source(TEST_SOURCE)
                .destination(TEST_DESTINATION)
                .authority_level(TEST_AUTHORITY_LEVEL)
                .build()
                .try_to_sysex8(&mut [
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                ], TEST_STREAM_ID)
                .unwrap(),
            &[
                sysex8::Message::builder()
                    .stream_id(TEST_STREAM_ID)
                    .group(TEST_GROUP)
                    .status(sysex8::Status::Begin)
                    .data(&[
                        0x7E, // universal sysex
                        0x7F, // Device ID: whole midi port
                        0x0D, // universal sysex sub-id 1: midi ci
                        0x13, // universal sysex sub-id 2: test protocol query
                        VERSION,
                        0b00111111, 0b01110001, 0b01101111, 0b00001110, // source
                        0b01011010, 0b00100111, 0b01011100, // destination
                    ])
                    .build(),
                sysex8::Message::builder()
                    .stream_id(TEST_STREAM_ID)
                    .group(TEST_GROUP)
                    .status(sysex8::Status::Continue)
                    .data(&[
                        0b01011101, // destination
                        0x19, // authority level
                        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 
                        0x08, 0x09, // test data
                    ])
                    .build(),
                sysex8::Message::builder()
                    .stream_id(TEST_STREAM_ID)
                    .group(TEST_GROUP)
                    .status(sysex8::Status::Continue)
                    .data(&[
                        0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 
                        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, // test data
                    ])
                    .build(),
                sysex8::Message::builder()
                    .stream_id(TEST_STREAM_ID)
                    .group(TEST_GROUP)
                    .status(sysex8::Status::Continue)
                    .data(&[
                        0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B,
                        0x1C, 0x1D, 0x1E, 0x1F, 0x20, 0x21, // test data
                    ])
                    .build(),
                sysex8::Message::builder()
                    .stream_id(TEST_STREAM_ID)
                    .group(TEST_GROUP)
                    .status(sysex8::Status::Continue)
                    .data(&[
                        0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 
                        0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, // test data
                    ])
                    .build(),
                sysex8::Message::builder()
                    .stream_id(TEST_STREAM_ID)
                    .group(TEST_GROUP)
                    .status(sysex8::Status::End)
                    .data(&[
                        0x2E, 0x2F, // test data
                    ])
                    .build(),
            ],
        );
    }
}