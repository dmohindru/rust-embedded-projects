# Rust Embedded Projects

This repository documents my hands-on learning path in **Embedded Rust** developing async firmware built with the [Embassy](https://embassy.dev) framework using two platforms:

- [**BBC micro:bit v2**](./microbit-v2/)
- [**STM32F3 Discovery**](./stm32f3discovery/)

Each board folder contains self-contained projects developed with a **TDD (Test-Driven Development)** approach: all logic that can run on the host is unit-tested, and hardware access is isolated in thin async tasks.

## Structure

```
microbit/
  README.md           → index and roadmap
  projectXX_name/     → each project with full notes, code, and tests
stm32f3discovery/
  README.md           → index and roadmap
  projectXX_name/     → each project with full notes, code, and tests
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
