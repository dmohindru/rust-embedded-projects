## Introduction

Example project to demonstrate using button to receive inputs.

## Features

There example features

- Two debounced buttons left and right.
- Max7219 led matrix display displaying single character of a string
- Pressing left button will scroll the text in left direction, wrapping to last character on reaching first char.
- Pressing right button will scroll the text in right direction, wrapping to first character on reaching last char.

## Mental model

Spi = hardware
Mutex<Spi> = safe shared access
SpiDevice = adds chip-select + locking
Driver = uses SpiDevice, ignores hardware details

Mutex ensures only one async task uses SPI at a time, and NoopRawMutex is just the lightweight mechanism that makes this safe in a single-core setup.

## Tasks

1. Left button task to toggle direction (left/right) will use Mutex to Direction
2. Right button task moves step cursor (delay value) in single direction will use Mutex to StepCursor
3. Timer task
   - Extracts delay value from step cursor (mutex)
   - Sleeps for delay amount of time
   - Extracts current direction from Direction mutex
   - Moves Frame cursor to appropriate direction (left/right)
   - Render's frame using MAX7219 driver

Global states

- Direction Mutex
- Step Cursor Mutex
