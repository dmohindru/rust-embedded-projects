# Project 1: Blink Factory — GPIO + Timer Foundations

Learn to control GPIO pins and schedule non-blocking timing tasks using Embassy.

## Goals
- Control all 8 LEDs with varying blink patterns.
- Button cycles through LED patterns.
- Implement async Embassy timers for non-blocking delays.

## Peripherals
- GPIO for LEDs
- EXTI for button input
- Timer for scheduling

## TDD Ideas
- Test pattern state transitions.
- Test debounce logic for button input.
- Verify LED state machine sequencing.