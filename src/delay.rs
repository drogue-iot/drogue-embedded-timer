use embedded_time::Timer;
use embedded_time::duration::Duration;
use embedded_time::fixed_point::FixedPoint;
use core::convert::TryFrom;

/// A blocking delay
pub struct Delay<'a, Clock>
    where Clock: embedded_time::Clock
{
    clock: &'a Clock
}

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