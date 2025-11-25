# Project 2 — Dodge Game

**Goal:** Implement a simple dodge game where player moves left/right with buttons and avoids falling bricks.

## Features
- 5×5 LED matrix gameplay
- Player movement via buttons
- Random falling blocks
- Touch logo to start/restart
- Speaker sound feedback per level/event

## Concepts
- Async timers for movement and game ticks
- Game loop and state machine
- PWM for sound generation

## TDD Focus
- Collision detection
- Score and level progression
- Sound pattern scheduling

## Embassy HAL Dependencies
- `embassy-time`, `embassy-nrf::gpio/pwm`
