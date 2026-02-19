## Project specifically to demo working with LED matrix

### [Project 1](./led_matrix_project1/)

**Move horizontal line left or right**

- There is a straight vertical line
- on pressing left button it move to left till left most column
- on pressing right button it move to right till right most column

### [Project 2](./led_matrix_project2/)

**Moving arrow in clockwise and anticlock wise direction**

- there is an arrow starting at center of led matrix towards edges
- pressing left button move it clock wise direction
- pressing right button move it anti clock wise direction

### [Project 3](./led_matrix_project3/)

**Moving arrow in clockwise and anticlock wise direction with timer**

- there is an arrow starting at center of led matrix towards edges
- pressing left button will toggle direction of arrow movement clockwise/anticlockwise.
- pressing right button will cycle between fast to slow movement.

### [Project 4](./led_matrix_project4/)

**Manual Text Scroll**

- At a given movement a single character of a text is show on led matrix
- Pressing left button will scroll text to right and the end of string it wraps around
- Pressing right button will scroll text to left and the start of string it wraps around

### [Project 5](./led_matrix_project5/)

**Text Scroll in clockwise and anticlock wise direction with timer**

- At a given movement a single character of a text is show on led matrix
- On a timer interrupt text is scrolled in either left or right direction as per state of left button pressed. Initial direction is right
- Pressing left button will toggle scroll direction between left/right
- Pressing right button will cycle between fast to slow movement.

### [Project 6](./led_matrix_project6/)

**Scrolling Text Animation**

- Display a scrolling text like Dhruv Mohindru
- On a timer interrupt text is scrolled in either left or right direction as per state of left button pressed. Initial direction is right
- Pressing left button will toggle scroll direction between left/right
- Pressing right button will cycle between fast to slow movement.

### Project 7

**Scrolling text with start/pause feature**

- Display a scrolling text like Dhruv Mohindru
- On a timer interrupt text is scrolled to left only if scrolling in not paused
- Pressing left button will toggle pause on/off.
- Pressing right button will cycle between fast to slow movement.

## Micro:bit v2 LED Matrix — Driver Concepts

### 1. Matrix Basics

- **Size:** 5×5 LEDs
- **Wiring:** Each row has **common anode**, each column has **common cathode**.
- **Multiplexing:** Only **one row is active at a time** to reduce pin usage and power.
  - Microcontroller sets one row HIGH at a time and drives column pins LOW to light LEDs.
  - After a short delay (~300µs), the row is cleared and next row is activated.
- **Persistence of Vision:** Human eye perceives continuous light if refresh rate >50–70 Hz.

### 2. Row & Column Logic

- **Common Anode Row:** Row HIGH delivers power; row LOW disables LEDs in that row.
- **Column Cathode:** Column LOW allows current to flow; column HIGH keeps LED off.
- **Clear row after rendering:** Prevents ghosting when the next row is activated.

### 3. Brightness Control

- **PWM / Bit-plane approach:**
  - A row is repeatedly activated multiple times per refresh cycle.
  - LED ON duration vs OFF duration controls apparent brightness.
  - Example: 50% brightness → LED ON for half of row refresh cycles.
- **Bit-plane technique:**
  - Frame buffer contains multiple “planes” per row.
  - Lower significance planes repeat fewer times, higher planes longer.
  - Supports finer brightness levels efficiently.

### 4. Framebuffer Logic

- **Central concept:** All LED updates happen via a software framebuffer.
- **Frame buffer contains:**
  - Pixel values (ON/OFF or brightness level)
  - Used by multiplexing loop to drive row/column pins.
- **Frame buffer updates:**
  - For animations, scrolling text, or brightness fades, compute frames programmatically.
  - Hand-coding pixels is inefficient and error-prone.
  - Recommended: generate frames via **Python or JavaScript scripts** and include as constants in Rust code.

### 5. Refresh Loop (Example Logic)

```rust
loop {
    for row in 0..5 {
        matrix.render_row(row);       // Activate row and set columns
        timer.after_micros(300).await;
        matrix.clear_row(row);        // Turn off row
    }
}
```

- Each iteration updates one row.
- Loop continuously for all rows to maintain persistence of vision.

### 6. Brightness & Animations

- **Per-row brightness:** Repeat row activation for `n` cycles proportional to brightness.
- **Fade in/out / smooth animation:** Update framebuffer values gradually, then let multiplexing loop render them.
- **Scrolling text:** Treat string as a stream of columns; shift framebuffer horizontally each cycle.

### 7. Summary of Key Concepts

| Concept                | Notes                                                                           |
| ---------------------- | ------------------------------------------------------------------------------- |
| Multiplexing           | One row active at a time                                                        |
| Common anode / cathode | Row HIGH powers, column LOW lights LED                                          |
| Framebuffer            | Stores pixel or brightness data                                                 |
| PWM / Bit-plane        | Controls brightness via intra-row on/off                                        |
| Persistence of vision  | Refresh rate ≥50–70 Hz prevents flicker                                         |
| Animation              | Update framebuffer programmatically (script-generated for large/complex frames) |

## Tool to use

Use this tool to generate pixel frame data
https://dot2pic.com/
