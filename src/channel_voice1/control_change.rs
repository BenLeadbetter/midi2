use crate::{
    channel_voice1::UMP_MESSAGE_TYPE,
    detail::{common_properties, schema},
};

pub(crate) const STATUS: u8 = 0b1011;

#[midi2_proc::generate_message(FixedSize, MinSizeUmp(1), MinSizeBytes(3))]
struct ControlChange {
    #[property(crate::utility::JitterReductionProperty)]
    jitter_reduction: Option<crate::utility::JitterReduction>,
    #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    ump_type: (),
    #[property(common_properties::ChannelVoiceStatusProperty<STATUS>)]
    status: (),
    #[property(common_properties::ChannelProperty)]
    channel: crate::numeric_types::u4,
    #[property(common_properties::GroupProperty)]
    group: crate::numeric_types::u4,
    #[property(common_properties::HybridSchemaProperty<
        crate::numeric_types::u7,
        schema::Bytes<0x00, 0x7F, 0x0>,
        schema::Ump<0x0000_7F00, 0x0, 0x0, 0x0>,
    >)]
    control: crate::numeric_types::u7,
    #[property(common_properties::HybridSchemaProperty<
        crate::numeric_types::u7,
        schema::Bytes<0x00, 0x0, 0x7F>,
        schema::Ump<0x0000_007F, 0x0, 0x0, 0x0>,
    >)]
    control_data: crate::numeric_types::u7,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        numeric_types::*,
        traits::{Channeled, Grouped},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn setters() {
        let mut message = ControlChange::new_arr();
        message.set_group(u4::new(0xA));
        message.set_channel(u4::new(0x7));
        message.set_control(u7::new(0x36));
        message.set_control_data(u7::new(0x37));
        assert_eq!(message, ControlChange([0x0, 0x2AB7_3637, 0x0, 0x0, 0x0]));
    }

    #[test]
    fn setters_bytes() {
        let mut message = ControlChange::new_arr_bytes();
        message.set_channel(u4::new(0x7));
        message.set_control(u7::new(0x36));
        message.set_control_data(u7::new(0x37));
        assert_eq!(message, ControlChange([0xB7, 0x36, 0x37]));
    }

    #[test]
    fn group() {
        assert_eq!(
            ControlChange::try_from(&[0x2AB7_3637_u32][..])
                .unwrap()
                .group(),
            u4::new(0xA),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            ControlChange::try_from(&[0x2AB7_3637_u32][..])
                .unwrap()
                .channel(),
            u4::new(0x7),
        );
    }

    #[test]
    fn channel_bytes() {
        assert_eq!(
            ControlChange::try_from(&[0xB7_u8, 0x36_u8, 0x37_u8][..])
                .unwrap()
                .channel(),
            u4::new(0x7),
        );
    }

    #[test]
    fn control() {
        assert_eq!(
            ControlChange::try_from(&[0x2AB7_3637_u32][..])
                .unwrap()
                .control(),
            u7::new(0x36),
        );
    }

    #[test]
    fn control_bytes() {
        assert_eq!(
            ControlChange::try_from(&[0xB7_u8, 0x36_u8, 0x37_u8][..])
                .unwrap()
                .control(),
            u7::new(0x36),
        );
    }

    #[test]
    fn control_data() {
        assert_eq!(
            ControlChange::try_from(&[0x2AB7_3637_u32][..])
                .unwrap()
                .control_data(),
            u7::new(0x37),
        );
    }

    #[test]
    fn control_data_bytes() {
        assert_eq!(
            ControlChange::try_from(&[0xB7_u8, 0x36_u8, 0x37_u8][..])
                .unwrap()
                .control_data(),
            u7::new(0x37),
        );
    }
}