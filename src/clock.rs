use embedded_time::clock::{Clock, Error};
use embedded_time::Instant;
use embedded_time::fraction::Fraction;
use embedded_time::duration::{Milliseconds, Seconds, Duration};
use embedded_time::duration::Generic;
use core::convert::{TryInto, TryFrom};
use cortex_m::interrupt::{Mutex, CriticalSection};
use embedded_time::fixed_point::FixedPoint;
use core::borrow::BorrowMut;
use core::cell::Cell;
use cortex_m::interrupt;
use core::marker::PhantomData;

pub struct Storage<Clock>
    where
        Clock: embedded_time::Clock,
{
    instant: Mutex<Cell<Option<Instant<Clock>>>>,
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

pub struct Ticker<'a, Clock: embedded_time::Clock, Timer, IrqClearer: Fn(&mut Timer)>
{
    timer: Timer,
    irq_clearer: IrqClearer,
    instant: &'a Storage<Clock>,
}

impl<'a, Clock: embedded_time::Clock, Timer, IrqClearer: Fn(&mut Timer)> Ticker<'a, Clock, Timer, IrqClearer> {
    fn new(timer: Timer, irq_clearer: IrqClearer, storage: &'a Storage<Clock>) -> Self {
        Self {
            timer,
            irq_clearer,
            instant: &storage,
        }
    }

    pub fn tick(&mut self) {
        self.instant.tick(Milliseconds(250u32));
        //self.instant.tick();
        (self.irq_clearer)(&mut self.timer);
    }
}

macro_rules! clock {
    ($name:ident, $ticker_type:ident, $per_second:literal, $per_tick:literal, $dur:expr) => {

        pub struct $name
        {
            instant: Storage<Self>,
        }

        impl $name {
            pub const fn new() -> Self
            {
                Self {
                    instant: Storage {
                        instant: cortex_m::interrupt::Mutex::new(core::cell::Cell::new(Option::None)),
                    },
                }
            }

            pub fn ticker<Timer, IrqClearer: Fn(&mut Timer)>(&self, timer: Timer, irq_clearer: IrqClearer) -> $ticker_type<Self, Timer, IrqClearer> {
                let i = &self.instant;
                $ticker_type::new(timer, irq_clearer, i)
            }
        }

        impl Clock for $name {
            type T = u32;
            const SCALING_FACTOR: Fraction = Fraction::new($per_tick, $per_second);

            fn try_now(&self) -> Result<Instant<Self>, Error> {
                Ok(self.instant.get())
            }
        }
        ticker!($ticker_type, $dur);
    }
}

macro_rules! ticker {
    ($name:ident, $tick:expr) => {
        pub struct $name<'a, Clock: embedded_time::Clock, Timer, IrqClearer: Fn(&mut Timer)>
        {
            timer: Timer,
            irq_clearer: IrqClearer,
            instant: &'a Storage<Clock>,
        }

        impl<'a, Clock: embedded_time::Clock, Timer, IrqClearer: Fn(&mut Timer)> $name<'a, Clock, Timer, IrqClearer> {
            fn new(timer: Timer, irq_clearer: IrqClearer, storage: &'a Storage<Clock>) -> Self {
                Self {
                    timer,
                    irq_clearer,
                    instant: &storage,
                }
            }

            pub fn tick(&mut self) {
                //self.instant.tick(Milliseconds(250u32));
                self.instant.tick( $tick );
                (self.irq_clearer)(&mut self.timer);
            }
        }
    }
}

clock!(MillisecondsClock1, MillisecondsTicker1, 1000, 1, Milliseconds(1u32));
clock!(MillisecondsClock10, MillisecondsTicker10, 1000, 10, Milliseconds(10u32));
clock!(MillisecondsClock50, MillisecondsTicker50, 1000, 50, Milliseconds(50u32));
clock!(MillisecondsClock100, MillisecondsTicker100, 1000, 100, Milliseconds(100u32));
clock!(MillisecondsClock200, MillisecondsTicker200, 1000, 200, Milliseconds(200u32));
clock!(MillisecondsClock250, MillisecondsTicker250, 1000, 250, Milliseconds(250u32));
clock!(MillisecondsClock500, MillisecondsTicker500, 1000, 500, Milliseconds(500u32));
clock!(SecondsClock, SecondsTicker, 1, 1, Seconds(1u32));
//ticker!(SecondsTicker, Seconds(1u32));
