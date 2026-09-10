# ELECFREAKS micro:bit Retro Arcade

### Hardware target: hw---n3

### MCU: nRF52833 (micro:bit V2)

### Display: 160x128 TFT

## Source:

pxt-arcade/libs/hw---n3/config.ts

## Pin Mapping

### Microbit-v2 pins

Arcade Shield INSR_LATCH
micro:bit edge connector P9
nRF52833 P0.09
| Shield Pin | Micro:bit edge pin | nRF52833 pin |
|------------|--------------------|--------------|
| INSR_LATCH | P9 | P0.09 |
| SR_CLK | P20 | P1.00 |
| INSR0_DATA | P14 | P0.01 |

---

### Serial input data bitmaps

**Byte 0 - Button data**
|bit n| button |
|-----|--------|
| 7 | GND |
| 6 | menu |
| 5 | btn_b |
| 4 | btn_a |
| 3 | btn_right |
| 2 | btn_down |
| 1 | btn_up |
| 0 | btn_left |

**note**: Config is active low, means

- bit value 0: pressed
- bit value 1: not pressed

---

**Byte 1 - Config data**

- Value: 0x30
- HW_CFG3 = 0
- HW_CFG2 = 0
- HW_CFG1 = 1 -> ST7735 (CFG1=0x603 or 0x12c2d)
- HW_CFG0 = 1 -> Not rotated

## TFT Display

// TODO Verify this

Display controller:
ST7735

Resolution:
160 x 128

SPI / control pins:

    SCK  = nRF P0.17
    MOSI = nRF P0.13
    MISO = nRF P0.01
    BL   = nRF P0.26
    DC   = nRF P0.10
    RST  = nRF P1.02

NOTE:
P0.01 is shared by: - button DATA - display MISO

    Need to understand how the hardware/software handles this
    shared connection.

## Audio

    Speaker / sound = nRF P0.00

## Jacdac

    Jacdac / accessibility pin = nRF P0.12

## Display Configuration

    DISPLAY_WIDTH  = 160
    DISPLAY_HEIGHT = 128
    DISPLAY_DELAY  = 300
    CLOCK_SPEED    = 32
    DISPLAY_TYPE   = 4242

    DISPLAY_CFG0 = 0x00000080
    DISPLAY_CFG1 = 0x00000603
    DISPLAY_CFG2 = 8

### Links

https://arcade.makecode.com/hardware/adding
https://github.com/microsoft/pxt-arcade-hardware-designs/tree/master/microbit-shield
