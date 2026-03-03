## Introduction

This folder contains generic/platform independent rust code

- various used full data structures
- platform independent driver code

## Contents

Following code assets are there in this folder

[**Buttons**](./src/button/)

- Debounced Button

[**Cursors**](./src/cursor/)

- Step cursor

[**Frames**](./src/frame/): Frame abstractions for Led Matrix

- Frame: Data structure for frame representation
- Frame Cursor: Data structure for scrolling between Frame array
- Frame Cursor circular: Data structure for scrolling and rotating at end/start of Frame array
- Frame Decoder: Decoder to read binary data and return Frame array

[**Display Drivers**](./src/display_driver/): Platform independent display drivers

- LedMatrix Driver: Driver for raw led matrix hardware
- Max7219 Driver: Driver to led matrix driver by chip Max7219
