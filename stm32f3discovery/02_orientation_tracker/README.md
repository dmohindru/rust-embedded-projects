# Project 2: Orientation Tracker — SPI Sensors

Integrate SPI sensors (accelerometer, gyroscope) and visualize tilt/orientation.

## Goals
- Read LIS302DL and L3GD20 via SPI.
- Compute tilt and orientation (roll/pitch).
- Display direction on LEDs.
- Stream sensor data via UART.

## Peripherals
- SPI
- GPIO (LED display)
- UART (data output)

## TDD Ideas
- Mock sensor data and verify orientation computation.
- Test data smoothing and filtering.