# Project 5: Sensor Fusion Dashboard — Full Integration

Integrate all onboard sensors and expose telemetry to a Rust CLI dashboard.

## Goals
- Read accelerometer, gyro, magnetometer, temperature.
- Compute orientation and heading.
- Stream telemetry as JSON or binary over UART.
- CLI app visualizes sensor data and controls settings (e.g., sampling rate).

## Peripherals
- SPI (accelerometer, gyro)
- I²C (magnetometer, DAC)
- UART (CLI telemetry)
- GPIO (LED indicators)
- Timer (sampling scheduler)

## TDD Ideas
- Unit test telemetry formatting.
- Validate sampling scheduler timing logic.
- Test orientation computation using mock sensor input.