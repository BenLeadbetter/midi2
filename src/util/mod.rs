use crate::{
    packet::Packet,
};

mod bounded;
mod truncate;
mod slice_data;

pub use bounded::Bounded;
pub use truncate::Truncate;
pub use slice_data::SliceData;

pub trait MessageTraits {
    const DUMMY: u8 = 0;
}

impl<T> MessageTraits for T where T:
    Clone +
    core::fmt::Debug +
    PartialEq +
    core::convert::TryFrom<Packet> +
    core::convert::Into<Packet> +
{}

#[cfg(test)]
macro_rules! message_traits_test {
    ($t:ty) => {
        use crate::util::MessageTraits;
        #[test]
        fn traits() {
            let _ = <Message as MessageTraits>::DUMMY;
        }
    };
}

#[cfg(test)]
pub(crate) use message_traits_test;