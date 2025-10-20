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
