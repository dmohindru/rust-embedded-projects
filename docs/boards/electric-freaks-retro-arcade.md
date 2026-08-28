# ELECFREAKS micro:bit Retro Arcade

### Hardware target: hw---n3

### MCU: nRF52833 (micro:bit V2)

### Display: 160x128 TFT

## Source:

pxt-arcade/libs/hw---n3/config.ts

### NOTE:

PIN_BTN_LEFT .. PIN_BTN_MENU are logical MakeCode button IDs,

NOT physical nRF52833 GPIO numbers.

Their mapping to individual 74HC165 bits is still to be determined.

## Buttons

Button interface:
DATA = nRF P0.01
CLOCK = nRF P1.00
LATCH = nRF P0.09

Logical buttons:
LEFT = MakeCode logical pin 1050
UP = MakeCode logical pin 1051
DOWN = MakeCode logical pin 1052
RIGHT = MakeCode logical pin 1053
A = MakeCode logical pin 1054
B = MakeCode logical pin 1055
MENU = MakeCode logical pin 1056

TODO:
Determine which 74HC165 output bit corresponds to each
logical button.

## TFT Display

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

## micro:bit Edge Connector

The Arcade Shield exposes:

    micro:bit P0 -> nRF P0.02
    micro:bit P1 -> nRF P0.03
    micro:bit P2 -> nRF P0.04

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

## A compact table version

| Hardware     | Signal    | nRF52833 pin | Notes                       |
| ------------ | --------- | -----------: | --------------------------- |
| Buttons      | DATA      |    **P0.01** | Serial button data          |
| Buttons      | CLOCK     |    **P1.00** | Button shift/register clock |
| Buttons      | LATCH     |    **P0.09** | Button latch                |
| Display      | SCK       |    **P0.17** | SPI clock                   |
| Display      | MOSI      |    **P0.13** | SPI data out                |
| Display      | MISO      |    **P0.01** | Shared with button DATA     |
| Display      | Backlight |    **P0.26** | GPIO                        |
| Display      | D/C       |    **P0.10** | Data/command                |
| Display      | RESET     |    **P1.02** | Display reset               |
| Speaker      | SOUND     |    **P0.00** | Audio output                |
| Jacdac       | TX        |    **P0.12** | Accessibility/Jacdac        |
| micro:bit P0 | —         |    **P0.02** | Edge connector              |
| micro:bit P1 | —         |    **P0.03** | Edge connector              |
| micro:bit P2 | —         |    **P0.04** | Edge connector              |

## Current understanding

### KNOWN

- MCU is nRF52833
- Display is 160x128
- Display interface uses SPI + GPIO
- Display pins are known
- Button interface uses DATA/CLOCK/LATCH
- Button interface is associated with a shift-register design
- Speaker pin is known
- micro:bit P0/P1/P2 mappings are known

### UNKNOWN

- Exact 74HC165 configuration
- Exact button -> shift-register-bit mapping
- Button active-high/active-low behaviour
- Exact ST7735 initialization sequence
- SPI mode / frequency used by the display
- Display orientation / MADCTL configuration
- Meaning of DISPLAY_CFG0/1/2
- Exact handling of P0.01 being shared by button DATA and display MISO
