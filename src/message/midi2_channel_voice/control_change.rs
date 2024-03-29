const OP_CODE: u32 = 0b1011;
const MIDI2_CHANNEL_VOICE_TYPE: u32 = 0x4;

#[midi2_proc::generate_message(Grouped, Channeled)]
struct ControlChange {
    ump_type: Property<
        NumericalConstant<MIDI2_CHANNEL_VOICE_TYPE>,
        UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>,
        (),
    >,
    status: Property<NumericalConstant<OP_CODE>, UmpSchema<0x00F0_0000, 0x0, 0x0, 0x0>, ()>,
    channel: Property<u4, UmpSchema<0x000F_0000, 0x0, 0x0, 0x0>, ()>,
    control: Property<u7, UmpSchema<0x0000_7F00, 0x0, 0x0, 0x0>, ()>,
    control_change_data: Property<u32, UmpSchema<0x0000_0000, 0xFFFF_FFFF, 0x0, 0x0>, ()>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            ControlChangeMessage::builder()
                .group(u4::new(0x3))
                .channel(u4::new(0x9))
                .control(u7::new(0x30))
                .control_change_data(0x2468_1012)
                .build(),
            Ok(ControlChangeMessage::Owned(ControlChangeOwned([
                0x43B9_3000,
                0x2468_1012,
                0x0,
                0x0
            ])))
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            ControlChangeMessage::from_data(&[0x43B9_3000, 0x2468_1012, 0x0, 0x0])
                .unwrap()
                .group(),
            u4::new(0x3),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            ControlChangeMessage::from_data(&[0x43B9_3000, 0x2468_1012, 0x0, 0x0])
                .unwrap()
                .channel(),
            u4::new(0x9),
        );
    }

    #[test]
    fn control() {
        assert_eq!(
            ControlChangeMessage::from_data(&[0x43B9_3000, 0x2468_1012, 0x0, 0x0])
                .unwrap()
                .control(),
            u7::new(0x30),
        );
    }

    #[test]
    fn control_change_data() {
        assert_eq!(
            ControlChangeMessage::from_data(&[0x43B9_3000, 0x2468_1012, 0x0, 0x0])
                .unwrap()
                .control_change_data(),
            0x2468_1012,
        );
    }
}
