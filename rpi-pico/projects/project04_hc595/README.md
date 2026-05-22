## Introduction

Example project to demonstrate use of hc595 serial shift register

## Features

- Simple demo example that demos a 8 bit binary counter on 8 bit led bar and data is pushed through hc959 serial shift register
- Demos the spi based driver for [hc595 device](../../../embedded-core/src/shift_register/hc595.rs)
- Two debounced buttons left and right.
- Left button task doesn't do much except for logging button pressed
- Right button task toggle display on/off
- timer task updates the counter and pushed to hc595 device
