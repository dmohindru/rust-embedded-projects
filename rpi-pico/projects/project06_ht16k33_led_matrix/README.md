## Introduction

Example project to demonstrate using Ht16K33 backed led matrix.

## Features

There example features

- Two debounced buttons left and right.
- Ht16K33 led matrix display displaying single character of a string
- Pressing left button will scroll the text in left direction, wrapping to last character on reaching first char.
- Pressing right button will scroll the text in right direction, wrapping to first character on reaching last char.

## Tasks

1. Left button task to toggle direction (left/right) will use Mutex to Direction
2. Right button task moves step cursor (delay value) in single direction will use Mutex to StepCursor
3. Timer task
   - Extracts delay value from step cursor (mutex)
   - Sleeps for delay amount of time
   - Extracts current direction from Direction mutex
   - Moves Frame cursor to appropriate direction (left/right)
   - Render's frame using MAX7219 driver

## Demo

![Demo Animation](./demo.gif)
