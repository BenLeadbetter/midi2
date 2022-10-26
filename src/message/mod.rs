use crate::packet::{Packet, PacketMethods};

pub mod midi1_channel_voice;
pub mod midi2_channel_voice;
pub mod system_common;
pub mod system_exclusive;
pub mod utility;

mod helpers;

fn write_type_to_packet(t: ux::u4, p: &mut Packet) {
    p.set_nibble(0, t);
}

fn write_group_to_packet(g: ux::u4, p: &mut Packet) {
    p.set_nibble(1, g);
}