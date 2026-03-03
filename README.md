# Rust Embedded Projects

This repository documents my hands-on learning path in **Embedded Rust** developing async firmware built with the [Embassy](https://embassy.dev) framework using following platforms:

- [**BBC micro:bit v2**](./microbit-v2/)
- [**STM32F3 Discovery**](./stm32f3discovery/)
- [**Raspberry Pi Pico**](./rpi-pico/)

## Structure

```
embedded-core/        → Contains generic platform independent code for various data structures and drivers
microbit-v2/          → Contains projects specific to microbit-v2 platform
stm32f3discovery/     → Contains projects specific to stm32 platform
rpi-pico/             → Contains projects specific to raspberry pi pico platform
```

## Tools

- Rust (2024 edition)
- Embassy framework
- probe-rs / cargo-embed
- defmt / rtt-target for logging
- tokio + clap for host-side CLI apps

```
cargo test                # run host-side tests
cargo embed               # flash micro:bit
```
