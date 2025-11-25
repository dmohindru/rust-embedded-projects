# Project 5 — Environmental Monitor

**Goal:** Build dual-mode environmental monitor using temperature and ambient sound sensors.

## Modes
- **Button A → Temperature Mode**
  - Display LED pattern for temperature
  - Log only temperature data
  - Buzzer when threshold exceeded

- **Button B → Ambient Noise Mode**
  - Display LED pattern for sound level
  - Log only sound data
  - Buzzer when threshold exceeded

## Features
- UART logging and CLI monitoring
- LED pattern visualization
- Periodic sampling and event-driven alerts

## Concepts
- PDM microphone input
- Temperature sensor sampling
- Multi-mode FSM
- Threshold logic and alarms

## TDD Focus
- Threshold + hysteresis tests
- Mode switching logic
- Logging buffer tests
