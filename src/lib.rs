#![cfg_attr(not(test), no_std)]

mod clock;
mod delay;

pub use clock::*;
pub use delay::*;

#[macro_export]
/// Wrap a HAL-specific CountDown into an embedded-time-centric CountDown
macro_rules! embedded_countdown {
    ($name:ident, $from_unit:ty, $to_unit:ty => ($arg:tt) $convert:tt) => {
        pub struct $name<CD: CountDown<Time = $to_unit>> {
            t: CD,
        }

        impl<CD: CountDown<Time = $to_unit>> $name<CD> {
            pub fn from(t: CD) -> Self {
                Self { t }
            }
        }

        impl<CD> embedded_hal::timer::CountDown for $name<CD>
        where
            CD: CountDown<Time = $to_unit>,
        {
            type Time = $from_unit;
            type Error = CD::Error;

            fn try_start<T>(&mut self, count: T) -> Result<(), Self::Error>
            where
                T: Into<Self::Time>,
            {
                let $arg: $from_unit = count.into();
                let to_count = $convert;
                self.t.try_start(to_count)
            }

            fn try_wait(&mut self) -> nb::Result<(), Self::Error> {
                self.t.try_wait()
            }
        }
    };
}
