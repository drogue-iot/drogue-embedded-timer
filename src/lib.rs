#![cfg_attr(not(test), no_std)]

mod clock;
pub use clock::*;

#[macro_export]
macro_rules! embedded_countdown {
    ($name:ident, $from_unit:ty, $to_unit:ty => ($arg:tt) $convert:tt) => {

        pub struct $name<CD: CountDown<Time=$to_unit>> {
            t: CD,
        }

        impl<CD: CountDown<Time=$to_unit>> $name<CD> {
            pub fn from(t: CD) -> Self {
                Self {
                  t,
                }
            }
        }

        impl<CD> embedded_hal::timer::CountDown for $name<CD>
            where CD: CountDown<Time=$to_unit>
        {
            type Time = $from_unit;

            fn start<T>(&mut self, count: T)
              where T: Into<Self::Time>
            {
                let $arg: $from_unit = count.into();
                let to_count = $convert;
                self.t.start( to_count );
            }

            fn wait(&mut self) -> nb::Result<(), void::Void> {
                self.t.wait()
            }
        }
    }
}
