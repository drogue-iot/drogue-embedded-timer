use embedded_time::Timer;
use embedded_time::duration::{Duration, Milliseconds, Microseconds};
use embedded_time::fixed_point::FixedPoint;
use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use core::convert::TryFrom;

/// A blocking delay
///#[derive(Copy, Clone)]
pub struct Delay<'a, Clock>
    where Clock: embedded_time::Clock
{
    clock: &'a Clock
}

impl<Clock> Clone for Delay<'_, Clock>
    where Clock: embedded_time::Clock {
    fn clone(&self) -> Self {
        Self {
            clock: self.clock,
        }
    }
}

impl<Clock> Copy for Delay<'_, Clock>
    where Clock: embedded_time::Clock { }



impl<'a, Clock> Delay<'a, Clock>
    where
        Clock: embedded_time::Clock,

{

    /// Construct a new Delay.
    /// Probably preferable to use Clock::delay(), though.
    pub fn new(clock: &'a Clock) -> Self {
        Self {
            clock,
        }
    }

    /// Perform a blocking delay for at least the specified amount of time.
    /// The actual amount of delay time is dependent on the precision of the underlying clock.
    pub fn delay<Delay>(&self, delay: Delay)
        where
            Delay: Duration + FixedPoint,
            Clock::T: TryFrom<Delay::T>,
    {
        let timer = Timer::new(self.clock, delay);
        let timer = timer.start().unwrap();
        timer.wait().unwrap();
    }
}

macro_rules! delay_impl {
    ($impl:ident, $func:ident, $vt:ident, $wt:ident) => {
            impl<'a, Clock> $impl<$vt> for Delay<'a, Clock>
            where
                Clock: embedded_time::Clock,
        {
            fn $func(&mut self, value: $vt) {
                self.delay($wt(value as u32))
            }
        }
    };
}

delay_impl!(DelayMs, delay_ms, u8, Milliseconds);
delay_impl!(DelayMs, delay_ms, u16, Milliseconds);
delay_impl!(DelayUs, delay_us, u8, Microseconds);
delay_impl!(DelayUs, delay_us, u16, Microseconds);

/*
impl<'a, Clock> DelayMs<u16> for Delay<'a, Clock>
    where
        Clock: embedded_time::Clock,
{
    fn delay_ms(&mut self, value: u16) {
        self.delay(Milliseconds(value as u32))
    }
}

impl<'a, Clock> DelayMs<u8> for Delay<'a, Clock>
    where
        Clock: embedded_time::Clock,
{
    fn delay_ms(&mut self, ms: u8) {
        self.delay(Milliseconds(ms as u32))
    }
}
 */