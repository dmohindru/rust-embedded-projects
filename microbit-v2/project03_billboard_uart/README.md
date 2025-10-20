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
