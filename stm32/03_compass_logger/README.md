# Project 3: Compass Logger — I²C + Magnetometer

Use the onboard I²C magnetometer to compute compass direction.

## Goals
- Read heading from LSM303DLHC magnetometer.
- Display N/E/S/W using LEDs.
- Log heading over UART.
- Add calibration routine via button press.

## Peripherals
- I²C (magnetometer)
- GPIO (LEDs)
- UART (logging)

## TDD Ideas
- Test compass heading calculation from raw data.
- Validate calibration algorithm.