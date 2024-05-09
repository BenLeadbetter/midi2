use crate::{detail::common_properties, ump_stream, ump_stream::UMP_MESSAGE_TYPE};

pub(crate) const STATUS: u16 = 0x21;

#[midi2_proc::generate_message(FixedSize, MinSizeUmp(1))]
struct EndOfClip {
    #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    ump_type: (),
    #[property(ump_stream::StatusProperty<STATUS>)]
    status: (),
    #[property(ump_stream::ConsistentFormatsProperty)]
    consistent_formats: (),
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn setters() {
        assert_eq!(
            EndOfClip::new_arr(),
            EndOfClip([0xF021_0000, 0x0, 0x0, 0x0])
        );
    }

    #[test]
    fn from_data() {
        assert_eq!(
            EndOfClip::try_from(&[0xF021_0000][..]),
            Ok(EndOfClip(&[0xF021_0000][..]))
        );
    }
}
