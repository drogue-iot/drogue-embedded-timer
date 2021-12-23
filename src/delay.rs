use ::core::convert::Infallible;
use core::convert::TryFrom;
use embedded_hal::delay::blocking::DelayUs;
use embedded_time::duration::{Duration, Microseconds};
use embedded_time::fixed_point::FixedPoint;
use embedded_time::Timer;

/// A blocking delay
///#[derive(Copy, Clone)]
pub struct Delay<'a, Clock>
where
    Clock: embedded_time::Clock,
{
    clock: &'a Clock,
}

impl<Clock> Clone for Delay<'_, Clock>
where
    Clock: embedded_time::Clock,
{
    fn clone(&self) -> Self {
        Self { clock: self.clock }
    }
}

impl<Clock> Copy for Delay<'_, Clock> where Clock: embedded_time::Clock {}

impl<'a, Clock> Delay<'a, Clock>
where
    Clock: embedded_time::Clock,
{
    /// Construct a new Delay.
    /// Probably preferable to use Clock::delay(), though.
    pub fn new(clock: &'a Clock) -> Self {
        Self { clock }
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

impl<'a, Clock> DelayUs for Delay<'a, Clock>
where
    Clock: embedded_time::Clock,
{
    type Error = Infallible;

    fn delay_us(&mut self, value: u32) -> Result<(), Infallible> {
        Ok(self.delay(Microseconds(value as u32)))
    }
}
