# Project 4: UART LED Controller — Host–Device Interaction

Build a Rust CLI app to control LEDs via UART commands.

## Goals

- MCU receives commands like `LED1 ON`, `LED3 OFF`, `SET 0b10101010`.
- CLI to set/read led status.
- Update LED states accordingly.
- Send ACK or ERR responses.
- CLI app can read/write over USB serial.

## Peripherals

- UART (USB-CDC via ST-Link)
- GPIO (8 LEDs)

## TDD Ideas

- Unit test command parser and dispatch logic.
- Mock UART input stream and validate LED state changes.
