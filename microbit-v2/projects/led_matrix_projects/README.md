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
