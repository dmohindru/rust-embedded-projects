## Introduction

Example project to demonstrate using button to receive inputs.

## Features

There would be

- A global state protected by mutex storing what last button was pressed.
- Two debounced buttons lets call A and B.
- A global state would store values either "Button A" or "Button B" which would be set by pressing button A and button B respectively.
- A periodic task which prints the current value of global state to console.
