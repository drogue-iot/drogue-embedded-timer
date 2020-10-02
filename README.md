# `drogue-embedded-timer`

When writing device drivers against `embedded-hal`, a wall can be hit when attempting to work with `CountDown` timers due to the `Time` associated type.

The `embedded-time` crate is attempting to homogenous the concept of time, clocks, durations and rates.
At this point the various HALs have not adopted `embedded-time`, so this crate provides a simple macro to help accomodate drivers needing a consistent view of timers.

## Converting HAL `CountDown` to `embedded-time`

Generic drivers should be written in terms of `CountDown` that uses `embedded-time` flavors of time.

Application writers attempting to provision a concrete instance of the aforementioned drivers can use this macro to convert their HAL's timer into an `embedded-time`-centric `CountDown`.

### Example

Use the `embedded_countdown!(...)` macro to define a new struct that can consume a HAL-specific timer, and wrap it into an `embedded-time` timer.

The macro takes a few arguments:

1. The name of the struct to create.
2. The exposed units (usually an `embedded-time` duration) expected by the driver.
3. The HAL's units expected by the built-in `CountDown` structure being wrapped.
4. The 1-argument conversion routine to handle the conversion.

```rust
embedded_countdown!(MsToHertzCountDown,
                embedded_time::duration::Milliseconds,
                stm32l4xx_hal::time::Hertz
                 => (ms) {
                        let hz: embedded_time::rate::Hertz = ms.to_rate().unwrap();
                        stm32l4xx_hal::time::Hertz(hz.0)
                } );
```

Once a structure has been defined, you can then use it:

```rust
let mut hal_hz_timer = Timer::tim16(device.TIM16, 1, clocks, &mut rcc.apb2);
let mut embedded_ms_timer = MsToHertzCountDown::from(hal_hz_timer);
```

Now the `embedded_ms_timer` is a `CountDown<Time=embedded_time::duration::Milliseconds>` and is no longer tied to a specific HAL implementation.

## Using straight `embedded-time` clocks and timers:

This crate provides mechanisms for driving an `embedded-time`-centric `Clock` to be able to create `embedded-time` Timers.

You decide the precision of clock you want to use, first.

The available precisions are:

* 1 millisecond
* 10 milliseconds
* 50 milliseconds
* 100 milliseconds
* 200 milliseconds
* 250 milliseconds
* 500 milliseconds
* 1 second

Each clock type has a related _Ticker_ type that must also be used:

```rust

use drogue_embedded_timer::{
  MillisecondsClock100,
  MillisecondsTicker100,
}
```

Define the clock as a static variable:

```rust
static CLOCK: MillisecondsClock100 = MillisecondsClock100::new();
```

Configure one of your chip's timers to match the `CLOCK` you selected:

```rust
// STM32L4xx configuration, yours may vary:
let mut tim15 = Timer::tim15(device.TIM15, 100, clocks, &mut rcc.apb2);
```

Enable the timer as an interrupt source:

```rust
tim15.listen(Event::TimeOut);
```

Obtain a _ticker_ from the `CLOCK` to be used in the ISR. The `ticker(...)` method takes two arguments:

1. Some object you can use to clear the timeout (otherwise unconstrained).
2. A function-like thing that can use the object in (1) above, to clear the timeout.

```rust
let ticker = CLOCK.ticker(
               tim15, 
               (|t| { t.clear_interrupt(Event::TimeOut); }) as fn(&mut Timer<TIM15>));
```

Using RTIC, you may wish to assign the _ticker_ into the shared resources object:

```rust
struct Resources {
    ticker: MillisecondsTicker100<'static, MillisecondsClock100, Timer<TIM15>, fn(&mut Timer<TIM15>)>,
    ...
}
```

However is appropriate, call `tick()` on the ticker each time the ISR fires.
In RTIC, it might look similar to:

```rust
#[task(binds = TIM15, priority = 15, resources = [ticker])]
fn ticker(mut ctx: ticker::Context) {
    ctx.resources.ticker.tick();
}
```

The ISR should be relative high priority to ensure time marches on.

# Creating timers

Once your clock is running and ticking over, you can create as many timers as you desire, using normal `embedded-time` functionality:

```rust
// effectively a blocking Delay type of action:
let my_timer = embedded_time::Timer::new(&CLOCK, Seconds(10u32));
let my_timer = my_timer.start().unwrap();
my_timer.wait().unwrap();
```
