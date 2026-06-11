# Driver Development order

### HC595 — Start Here

74HC595 Shift Register

### HC165 — Perfect Follow-up

74HC165 Shift Register

### TM1637 — Great Timing Challenge

TM1637 7-Segment Display Driver

- Bit banging
- Raspberry pi pico PIO

### HT16K33 — Excellent I2C Display Driver

HT16K33 LED Driver

This is where driver design becomes more architectural.

Great because:

- proper I2C device
- internal RAM
- display state model
- blinking/brightness config
- matrix abstractions

### GJD 1602 IIC LCD

Likely a PCF8574 backpack over HD44780 LCD.

Usually these modules are:

HD44780 LCD
connected through PCF8574 I/O Expander

This is surprisingly educational because you’ll learn:

layered drivers
transport abstraction

### TM1638 — Best “Mini System”

TM1638 LED and Key Display Driver

This is probably the richest learning device in your list.

Why it’s fantastic:

LEDs
seven segment display
button scanning
bidirectional communication
state synchronization
