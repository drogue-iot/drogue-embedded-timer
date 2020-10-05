use core::{cell::Cell, convert::TryFrom};
use cortex_m::interrupt::{self, Mutex};
use embedded_time::{
    clock::{Clock, Error},
    duration::{Duration, Milliseconds, Seconds},
    fixed_point::FixedPoint,
    fraction::Fraction,
    Instant,
};

struct Storage<Clock>
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
        interrupt::free(|cs| {
            let instant = self.instant.borrow(&cs);
            let i = instant.get();

            instant.replace(Some(if let Some(i) = i {
                i.checked_add(duration).unwrap()
            } else {
                Instant::new(Clock::T::from(0u32))
            }));
        });
    }

    fn get(&self) -> Result<Instant<Clock>, Error> {
        interrupt::free(|cs| {
            let instant = self.instant.borrow(&cs).get();
            if let Some(instant) = instant {
                Ok(instant)
            } else {
                Err(Error::NotRunning)
            }
        })
    }
}

macro_rules! clock {
    ($name:ident, $ticker_type:ident, $dur:expr, $scaling_factor:expr) => {
        pub struct $name {
            instant: Storage<Self>,
        }

        impl $name {
            pub const fn new() -> Self {
                Self {
                    instant: Storage {
                        instant: Mutex::new(Cell::new(Option::None)),
                    },
                }
            }

            pub fn ticker<Timer, IrqClearer: Fn(&mut Timer)>(
                &self,
                timer: Timer,
                irq_clearer: IrqClearer,
            ) -> $ticker_type<Self, Timer, IrqClearer> {
                $ticker_type::new(timer, irq_clearer, &self.instant)
            }
        }

        impl Clock for $name {
            type T = u32;
            const SCALING_FACTOR: Fraction = $scaling_factor;

            fn try_now(&self) -> Result<Instant<Self>, Error> {
                self.instant.get()
            }
        }
        ticker!($ticker_type, $dur);
    };
}

macro_rules! ticker {
    ($name:ident, $tick:expr) => {
        pub struct $name<'a, Clock: embedded_time::Clock, Timer, IrqClearer: Fn(&mut Timer)> {
            timer: Timer,
            irq_clearer: IrqClearer,
            instant: &'a Storage<Clock>,
        }

        impl<'a, Clock: embedded_time::Clock, Timer, IrqClearer: Fn(&mut Timer)>
            $name<'a, Clock, Timer, IrqClearer>
        {
            fn new(timer: Timer, irq_clearer: IrqClearer, storage: &'a Storage<Clock>) -> Self {
                Self {
                    timer,
                    irq_clearer,
                    instant: &storage,
                }
            }

            pub fn tick(&mut self) {
                //self.instant.tick(Milliseconds(250u32));
                self.instant.tick($tick);
                (self.irq_clearer)(&mut self.timer);
            }
        }
    };
}

clock!(
    MillisecondsClock1,
    MillisecondsTicker1,
    Milliseconds(1u32),
    Fraction::new(1, 1000)
);
clock!(
    MillisecondsClock2,
    MillisecondsTicker2,
    Milliseconds(2u32),
    Fraction::new(1, 500)
);
clock!(
    MillisecondsClock5,
    MillisecondsTicker5,
    Milliseconds(5u32),
    Fraction::new(1, 200)
);
clock!(
    MillisecondsClock10,
    MillisecondsTicker10,
    Milliseconds(10u32),
    Fraction::new(1, 100)
);
clock!(
    MillisecondsClock25,
    MillisecondsTicker25,
    Milliseconds(10u32),
    Fraction::new(1, 50)
);
clock!(
    MillisecondsClock50,
    MillisecondsTicker50,
    Milliseconds(50u32),
    Fraction::new(1, 20)
);
clock!(
    MillisecondsClock100,
    MillisecondsTicker100,
    Milliseconds(100u32),
    Fraction::new(1, 10)
);
clock!(
    MillisecondsClock200,
    MillisecondsTicker200,
    Milliseconds(200u32),
    Fraction::new(1, 5)
);
clock!(
    MillisecondsClock250,
    MillisecondsTicker250,
    Milliseconds(250u32),
    Fraction::new(1, 4)
);
clock!(
    MillisecondsClock500,
    MillisecondsTicker500,
    Milliseconds(500u32),
    Fraction::new(1, 2)
);
clock!(
    SecondsClock1,
    SecondsTicker1,
    Seconds(1u32),
    Fraction::new(1, 1)
);
clock!(
    SecondsClock30,
    SecondsTicker30,
    Seconds(30u32),
    Fraction::new(30, 1)
);
clock!(
    SecondsClock60,
    SecondsTicker60,
    Seconds(60u32),
    Fraction::new(60, 1)
);
