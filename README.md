# Rust Embedded Projects

This repository documents my hands-on learning path in **Embedded Rust** using two platforms:

- **BBC micro:bit v2** — async firmware built with the [Embassy](https://embassy.dev) framework.
- **STM32F3 Discovery** — projects to follow later.

Each board folder contains self-contained projects developed with a **TDD (Test-Driven Development)** approach: all logic that can run on the host is unit-tested, and hardware access is isolated in thin async tasks.

## Structure

```
microbit/
  README.md           → index and roadmap
  projectXX_name/     → each project with full notes, code, and tests
stm32f3discovery/
  (coming soon)
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
