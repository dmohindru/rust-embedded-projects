### Introduction

This project will demo the coordination between button press updating display LED Matrix.

### Resources used

- Push button
- Led Matrix
- Mutex

https://blog.theembeddedrustacean.com/sharing-data-among-tasks-in-rust-embassy-synchronization-primitives

how to create two dimensional array uint8
vectors in embedded world

### Solution approach

### Links

https://hackaday.io/page/21907-test-driven-embedded-rust-development-tutorial

```text
micro:bit       MAX7219

P0.13  -------> CLK
P0.15  -------> DIN
P0.14  -------> CS
3V     -------> VCC
GND    -------> GND
```

micro:bit MAX7219

P0.13 → CLK
P0.15 → DIN
P0.12 → CS
3V → VCC
GND → GND

7. Practical rule

When using Embassy async drivers on nRF52:
SPI → use SPI3
I2C → use TWIM
UART → use UARTE

Basically:

Always prefer the DMA-enabled peripherals.
