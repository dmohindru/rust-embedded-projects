## Introduction

Example project to demonstrate using button to receive inputs.

## Features

There would be

- A global state protected by mutex storing what last button was pressed.
- Two debounced buttons lets call A and B.
- A global state would store values either "Button A" or "Button B" which would be set by pressing button A and button B respectively.
- A periodic task which prints the current value of global state to console.

## Mental model

Spi = hardware
Mutex<Spi> = safe shared access
SpiDevice = adds chip-select + locking
Driver = uses SpiDevice, ignores hardware details

Mutex ensures only one async task uses SPI at a time, and NoopRawMutex is just the lightweight mechanism that makes this safe in a single-core setup.

## Logic Analyzer

CH2 -> Chip Select
SCK -> Ch7
MOSI -> Ch5
