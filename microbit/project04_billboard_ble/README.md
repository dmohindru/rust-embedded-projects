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
