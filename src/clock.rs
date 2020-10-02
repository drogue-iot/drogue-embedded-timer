use embedded_time::clock::{Clock, Error};
use embedded_time::Instant;
use embedded_time::fraction::Fraction;
use embedded_time::duration::{Milliseconds, Duration};
use embedded_time::duration::Seconds;
use embedded_time::duration::Generic;
use core::convert::{TryInto, TryFrom};
use cortex_m::interrupt::{Mutex, CriticalSection};
use embedded_time::fixed_point::FixedPoint;
use core::borrow::BorrowMut;
use core::cell::Cell;
use cortex_m::interrupt;

struct Storage<Clock: embedded_time::Clock> {
    instant: Mutex<Cell<Option<Instant<Clock>>>>
}

impl<Clock> Storage<Clock>
    where
        Clock: embedded_time::Clock,
{
    fn tick<Dur>(&self, duration: Dur)
        where
            Dur: Duration + FixedPoint,
            Clock::T: TryFrom<Dur::T>,
    {
        unsafe {
            interrupt::free(|cs| {
                let instant = self.instant.borrow(&cs);
                let i = instant.get();
                let i = if i.is_some() {
                    Some(
                        i.unwrap().checked_add(duration).unwrap()
                    )
                } else {
                    Some(Instant::new(Clock::T::from(0u32)))
                };

                instant.replace(i);
            });
        }
    }

    fn get(&self) -> Instant<Clock>
    {
        unsafe {
            interrupt::free(|cs| {
                let i = self.instant.borrow(cs).get();
                let j = if i.is_some() {
                    i.unwrap()
                } else {
                    Instant::new(Clock::T::from(0u32))
                };
                j
            })
        }
    }
}

pub struct Ticker<'a, Timer, IrqClearer: Fn(&mut Timer)>
{
    timer: Timer,
    irq_clearer: IrqClearer,
    instant: &'a Storage<InterruptDriverClock>,
}

impl<'a, Timer, IrqClearer: Fn(&mut Timer)> Ticker<'a, Timer, IrqClearer> {
    fn new(timer: Timer, irq_clearer: IrqClearer, storage: &'a Storage<InterruptDriverClock>) -> Self {
        Self {
            timer,
            irq_clearer,
            instant: &storage,
        }
    }

    pub fn tick(&mut self) {
        self.instant.tick(Milliseconds(250u32));
        (self.irq_clearer)(&mut self.timer);
    }
}

pub struct InterruptDriverClock {
    instant: Storage<InterruptDriverClock>
}

impl InterruptDriverClock {
    pub const fn new() -> Self {
        Self {
            instant: Storage {
                //instant: Mutex::new(Cell::new( Instant::new(0)))
                instant: Mutex::new(Cell::new( Option::None ) )
            }
        }
    }

    pub fn ticker<Timer, IrqClearer: Fn(&mut Timer)>(&self, timer: Timer, irq_clearer: IrqClearer) -> Ticker<Timer,IrqClearer> {
        Ticker {
            timer: timer,
            irq_clearer: irq_clearer,
            instant: &self.instant
        }
    }

}

impl Clock for InterruptDriverClock {
    type T = u32;
    const SCALING_FACTOR: Fraction = Fraction::new(1, 4);

    fn try_now(&self) -> Result<Instant<Self>, Error> {
        Ok(self.instant.get())
    }
}


/*
macro_rules! clock {
    ($slices_per_second:literal, $slices_per_tick:literal) => {
        nothing
    }
}

macro_rules! ms_clock {
    ($precision:literal) => {
        clock!( 1000, $precision )
    }
}

macro_rules! seconds_clock {
    ($precision:literal) => {
        clock!( 1, $precision )
    }

}
 */