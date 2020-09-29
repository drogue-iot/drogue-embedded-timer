# `drogue-embedded-timer`

When writing device drivers against `embedded-hal`, a wall can be hit when attempting to work with `CountDown` timers due to the `Time` associated type.

The `embedded-time` crate is attempting to homogenous the concept of time, clocks, durations and rates.
At this point the various HALs have not adopted `embedded-time`, so this crate provides a simple macro to help accomodate drivers needing a consistent view of timers.

## Usage

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


