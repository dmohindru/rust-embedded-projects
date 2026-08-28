## Introduction

This folder contains generic/platform independent rust code

- various used full data structures
- platform independent driver code

## Contents

Following code assets are there in this folder

[**Input Device**](./src/input_device/)

- [Debounced Button](./src/input_device/button/): Software debounced input button

- [Nunchuk](./src/input_device/nunchuk/): Wii nunchuk device

[**Cursors**](./src/cursor/)

- Step cursor

[**Frames**](./src/frame/): Frame abstractions for Led Matrix

- Frame: Data structure for frame representation
- Frame Cursor: Data structure for scrolling between Frame array
- Frame Cursor circular: Data structure for scrolling and rotating at end/start of Frame array
- Frame Decoder: Decoder to read binary data and return Frame array

[**Display Drivers**](./src/display_driver/): Platform independent display drivers

- [LedMatrix Driver](./src/display_driver/led_matrix_driver.rs): Driver for raw led matrix hardware
- [Max7219 Driver](./src/display_driver/max7219_driver.rs): Driver to led matrix driver by chip Max7219
- [HT16K33 Driver](./src/display_driver/ht16k33_driver.rs): Driver to led matrix driver by chip HT16K33
- [SSD1306 Driver](./src/display_driver/ssd1306_driver/): Driver for 128X64 SSD1306 OLED display
