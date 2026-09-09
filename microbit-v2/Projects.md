# Project 1 — Compass + Level Checker + Data Logger

**Goal:** Build a dual-mode app where Button A activates compass mode and Button B activates level checker.  
Also log accelerometer and magnetometer data over UART.

## Features

- Compass: LED direction indicators based on magnetometer
- Level Checker: tilt-based LED visualization using accelerometer
- UART Data Logger: async streaming of sensor values
- Buttons for mode switching

## Concepts

- Async task orchestration
- I²C communication
- State machine for modes
- UART communication

## TDD Focus

- Mode FSM transitions
- LED pattern mapping
- Data formatting for UART

## Embassy HAL Dependencies

- `embassy-nrf` (GPIO, I²C, UART)
- `lis3dh` accelerometer driver
- `mmc5603nj` magnetometer driver

# Project 2 — Dodge Game

**Goal:** Implement a simple dodge game where player moves left/right with buttons and avoids falling bricks.

## Features

- 5×5 LED matrix gameplay
- Player movement via buttons
- Random falling blocks
- Touch logo to start/restart
- Speaker sound feedback per level/event

## Concepts

- Async timers for movement and game ticks
- Game loop and state machine
- PWM for sound generation

## TDD Focus

- Collision detection
- Score and level progression
- Sound pattern scheduling

## Embassy HAL Dependencies

- `embassy-time`, `embassy-nrf::gpio/pwm`

# Project 3 — Billboard UART

**Goal:** Send up to 7-character words from a Rust CLI over UART and display them scrolling on the LED matrix.

## Features

- UART-based message reception
- LED text scrolling
- Host CLI app built in Rust (tokio + clap)

## Concepts

- UART communication
- Protocol design and framing
- Shared logic crate between host and microbit

## TDD Focus

- Message serialization/deserialization
- Scrolling text buffer
- CLI argument parsing

# Project 4 — Billboard BLE

**Goal:** Android app sends up to 7-character words over BLE to micro:bit which displays them as scrolling text.

## Features

- BLE GATT service on micro:bit
- Android client with Kotlin coroutines
- Reuse logic from UART version

## Concepts

- BLE peripheral setup
- Characteristic read/write
- Rate limiting and packet parsing

## TDD Focus

- BLE message framing
- Shared parser module from Project 3

[text](projects/project06_env_monitor) [text](projects/project06_env_monitor/README.md)
