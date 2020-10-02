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
use core::marker::PhantomData;

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

pub struct Ticker<'a, TIM, Timer, IrqClearer: Fn(&mut Timer)>
{
    timer: Timer,
    irq_clearer: IrqClearer,
    instant: &'a Storage<SoftClock<TIM>>,
}

impl<'a, TIM, Timer, IrqClearer: Fn(&mut Timer)> Ticker<'a, TIM, Timer, IrqClearer> {
    fn new(timer: Timer, irq_clearer: IrqClearer, storage: &'a Storage<SoftClock<TIM>>) -> Self {
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

pub struct SoftClock<TIM>
{
    instant: Storage<SoftClock<TIM>>,
}

impl<TIM> SoftClock<TIM> {
    pub const fn new() -> Self
    {
        Self {
            instant: Storage {
                instant: Mutex::new(Cell::new(Option::None)),
            },
        }
    }

    pub fn ticker<Timer, IrqClearer: Fn(&mut Timer)>(&self, timer: Timer, irq_clearer: IrqClearer) -> Ticker<TIM, Timer, IrqClearer> {
        let i = &self.instant;
        Ticker::new(timer, irq_clearer, i)
    }
}

impl<TIM> Clock for SoftClock<TIM> {
    type T = u32;
    const SCALING_FACTOR: Fraction = Fraction::new(1, 4);

    fn try_now(&self) -> Result<Instant<Self>, Error> {
        Ok(self.instant.get())
    }
}



/*
#[macro_export]
macro_rules! clock {
    ($TIM:ident, $slices_per_second:literal, $slices_per_tick:literal) => {

        use embedded_time::fraction::Fraction;
        use embedded_time::clock::Clock;

        impl Clock for SoftClock<$TIM> {
            type T = u32;
            const SCALING_FACTOR: Fraction = Fraction::new(1, 4);

            fn try_now(&self) -> Result<Instant<Self>, Error> {
                Ok(self.instant.get())
            }
        }
     }
}

 */

/*
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