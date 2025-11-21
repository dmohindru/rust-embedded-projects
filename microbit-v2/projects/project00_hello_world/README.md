## Introduction

A basic hello world program for getting started with embassy framework. This program with log two different hello world strings at different intervals demonstrating embassy's async runtime.

## Dependencies Overview

### Core Embassy Crates

- **embassy-executor** - Provides the async executor/runtime that allows us to write async Rust code for microcontrollers
- **embassy-time** - Time utilities for delays and intervals in async context
- **embassy-nrf** - Hardware Abstraction Layer (HAL) for Nordic nRF microcontrollers (we're using nrf52833 which is in microbit v2)

### Board Support

- **microbit-v2** - Board support crate for microbit v2, provides convenient access to pins and peripherals

### Logging & Debugging

- **defmt** - Efficient, zero-allocation logging framework ideal for embedded systems
- **defmt-rtt** - Real-Time Transport backend for defmt, allows live debugging via J-Link/debugger
- **panic-probe** - Panic handler that captures panic information and sends it via RTT for debugging

### System

- **cortex-m** - Low-level access to Cortex-M processor features (used by Embassy)

## Build Profile

Release profile is optimized for:

- Size optimization (opt-level = "z")
- Link Time Optimization (LTO) for smaller binaries
- Single codegen unit for maximum optimization

## memory.x - Linker Script

The `memory.x` file defines the memory layout for the linker when building for the nRF52833 (microbit v2):

### Memory Organization

- **FLASH**: 512 KB starting at address 0x00000000

  - Used for program code and read-only data
  - Starts at 0x00000000 (can be adjusted if bootloader is used)

- **RAM**: 128 KB starting at address 0x20000000
  - Used for stack, heap, and global variables
  - Stack grows downward from the top of RAM
  - `_stack_start` is set to the end of RAM

### Key Symbols

- `_stack_start` - Stack grows downward from this address (end of RAM)
- `_sdata` - Customizable location for `.data` section (optional)
- `_sbss` - Customizable location for `.bss` section (optional)

This linker script is required by Rust's embedded build system to properly link the binary.

## build.rs - Build Script

The `build.rs` file runs before compilation and emits build directives (lines starting with `cargo:`) to configure linking:

- **cargo:rustc-link-search** — Adds OUT_DIR to the linker's search path so it can find `memory.x`
- **cargo:rerun-if-changed** — Re-runs the build script only when `memory.x` changes (optimization)
- **cargo:rustc-link-arg** — Passes `-Tmemory.x` to the linker, specifying the linker script

The script also copies `memory.x` into the build output directory, making it robust for workspace builds.

## .cargo/config.toml - Cargo Configuration

The `.cargo/config.toml` file configures the embedded build environment:

### Target

- **Target triple**: `thumbv7em-none-eabihf`
  - `thumbv7em` — ARM Cortex-M4 (the nRF52833 processor)
  - `none` — Bare metal (no OS)
  - `eabihf` — Hardware floating point support

### Build Settings

- **runner** — Sets `probe-rs` as the default runner for quick flash-and-execute workflow. The project config uses an explicit probe selector to target the BBC micro:bit probe connected to the host:

  `probe-rs run --probe 0d28:0204 --chip nRF52833_xxAA`

  Replace `0d28:0204` with your probe's VID:PID or full probe id if you have multiple probes connected.

- **linker** — Uses `arm-none-eabi-gcc` (from ARM GNU Embedded Toolchain)
- **rustflags** — Additional linker arguments:
  - `-Wl,--verbose` — Print linker invocation for debugging
  - `-Wl,--fatal-warnings` — Fail on linker warnings
  - `-Wl,--gc-sections` — Remove unused code sections (size optimization)
  - `-fuse-ld=bfd` — Use GNU linker for compatibility

### Profile Optimization

- **dev** — Fast compilation with debug info for development
- **release** — Size-optimized with LTO for production

## Understanding `.cargo/config.toml` (short tutorial)

This project includes a reasonably explicit `.cargo/config.toml`. Below is a short explanation of the common sections and what they're used for.

- `[build]`

  - `target` sets the default compilation target triple (e.g. `thumbv7em-none-eabihf`). This controls which Rust target libraries are used and is typically required for bare-metal builds.

- `[target.<triple>]` or `[target.'cfg(...)']`

  - Target-specific options. Use the exact triple (e.g. `thumbv7em-none-eabihf`) to scope settings to one target, or a `cfg(...)` selector to apply to a group of targets.
  - Common settings here: `runner` (how to flash/run), `linker` (which native linker to call), and other per-target options.

- `runner`

  - Command used when you run `cargo run`/`cargo run --target ...`. We use `probe-rs` here to flash and start the program. You can include `--probe <id>` and `--chip <chip>` to make the runner deterministic.

- `linker` and `rustflags`

  - `linker` chooses the native linker executable (commonly `arm-none-eabi-gcc` for GCC toolchain).
  - `rustflags` lets you pass additional flags to the linker (e.g. `--gc-sections` to strip unused code, `--fatal-warnings` to fail on warnings). These are useful for controlling binary size and link behavior.

- `[env]`

  - Sets environment variables for the build and runner. This is a handy place to set `DEFMT_LOG` so host-side tools/readers and the runtime use the same logging level without editing source code.
  - Example in this project: `DEFMT_LOG = "trace"` enables maximum defmt verbosity for debugging.

- `[profile.dev]` / `[profile.release]`
  - Customize optimization and debug info levels per profile. For embedded projects the `release` profile is often tuned for size (`opt-level = "z"`, `lto = true`).

Quick tutorial — typical workflow

1. Ensure you have the Rust target and native toolchain installed:

```bash
rustup target add thumbv7em-none-eabihf
# Install GNU Arm Embedded toolchain (if you prefer GCC)
sudo apt install gcc-arm-none-eabi  # or your platform's package manager
```

2. Connect your board and verify probe-rs detects it:

```bash
probe-rs list
```

3. Build and flash using the configured runner:

```bash
cargo run --target thumbv7em-none-eabihf
```

If you rely on a specific probe, adjust the `runner` field in `.cargo/config.toml` to include `--probe <id>` (we used `0d28:0204` in this project for the BBC micro:bit).

This README documents the intended defaults used in the project. If you prefer a minimal configuration, a shorter `.cargo/config.toml` can be used — accept the defaults and only set `runner` and `target` — but for deterministic CI and reproducible release builds the fuller config is recommended.

## Building & running for multiple targets

If your `.cargo/config.toml` contains multiple target sections (call them `A` and `B`), the commands below build or run for the selected target:

- Build for target A (debug):

```bash
cargo build --target A
```

- Build for target B (release):

```bash
cargo build --target B --release
```

- Build and run (uses the `runner` configured for that target):

```bash
cargo run --target A
cargo run --target B
```

Notes and tips:

- `cargo build --target <triple>` only cross-compiles and writes the artifacts to `target/<triple>/(debug|release)/`.
- `cargo run --target <triple>` invokes the configured `runner` for that target. If no `runner` is configured for the target, `cargo run` will error — you can still use `cargo build` and then run the flashing tool manually on the produced artifact.
- Make sure the Rust target is installed:

```bash
rustup target list --installed
rustup target add <target-triple>
```

- Ensure a native linker is present if your config specifies one (for example `arm-none-eabi-gcc`):

```bash
which arm-none-eabi-gcc && arm-none-eabi-gcc --version
# or install via your package manager, e.g. on Debian/Ubuntu:
sudo apt install gcc-arm-none-eabi
```

- If you prefer to run the flashing tool manually, call `probe-rs` (or your runner) with the built artifact:

```bash
probe-rs run --probe <id> --chip <chip> target/<triple>/debug/<binary-name>
```

- Artifact locations:
  - Debug: `target/<triple>/debug/<package-name>`
  - Release: `target/<triple>/release/<package-name>`

Common troubleshooting:

- Missing linker or `rustc` errors about `cc`/`ar`: install the GNU Arm Embedded toolchain or switch to `rust-lld`.
- `cargo run` says no runner: add a `runner` to the appropriate `[target...]` section in `.cargo/config.toml` or run the flashing tool manually.

## TODO

- .vscode/tasks.json (Build tasks)
- .vscode/launch.json (Debugger launch configuration)
✓ Example main.rs with Embassy async runtime (completed)

## Example main.rs — Embassy Async Runtime

The `src/main.rs` file demonstrates the Embassy async runtime with two concurrent tasks:

### Architecture

- **#![no_std] #![no_main]** — Bare-metal Rust: no standard library, we provide the entry point.
- **task_fast()** — Async task logging every 500ms using `Timer::after()`.
- **task_slow()** — Async task logging every 2 seconds, also async.
- **main()** — Marked with `#[embassy_executor::main]`, initializes the executor and spawns both tasks. The executor keeps them running concurrently.

### Key components

- **embassy_executor::Spawner** — Used to spawn tasks onto the executor.
- **embassy_nrf::init()** — Initializes nRF52833 peripherals (for this template, we use default config).
- **embassy_time::Timer** — Provides async-friendly delays (no blocking).
- **defmt::info!()** — Logs messages via defmt (efficient, zero-allocation logging).
- **panic_probe** — Captures panics and sends them via RTT.

When you run this program, you should see:
- "=== Embassy Async Runtime Demo ===" (once at startup)
- "Fast task: Hello from Embassy! (500ms interval)" (every 500ms)
- "Slow task: Hello from microbit v2! (2s interval)" (every 2 seconds)

Both tasks run concurrently thanks to Embassy's executor; they don't block each other.

## Building and flashing manually

If you prefer not to use `cargo run`, you can build and flash in two steps:

### Step 1: Build

```bash
cargo build --target thumbv7em-none-eabihf
# or for release (optimized for size):
cargo build --target thumbv7em-none-eabihf --release
```

The binary is produced at:
- Debug: `target/thumbv7em-none-eabihf/debug/project00_hello_world`
- Release: `target/thumbv7em-none-eabihf/release/project00_hello_world`

### Step 2: Flash and run with probe-rs

```bash
probe-rs run --probe 0d28:0204 --chip nRF52833_xxAA target/thumbv7em-none-eabihf/debug/project00_hello_world
```

Replace `0d28:0204` with your probe's VID:PID if different. Omit `--probe` if you have a single probe connected:

```bash
probe-rs run --chip nRF52833_xxAA target/thumbv7em-none-eabihf/debug/project00_hello_world
```

### Viewing logs

The program outputs logs via defmt over RTT (Real-Time Transport). To see them:

1. **During `cargo run` or `probe-rs run`**: Most tools automatically display logs in the terminal.
2. **Manual inspection**: Use a defmt viewer:

```bash
# Install defmt CLI if not present
cargo install defmt-print
# View logs from a running program
defmt-print /dev/ttyACM0  # Linux path; adjust for your system
```
