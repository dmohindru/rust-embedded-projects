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

## Logic Analyzer

CH2 -> Chip Select
SCK -> Ch7
MOSI -> Ch5
