## Workspace for micro:bit v2 projects and shared drivers.

## BBC micro:bit V2 — Onboard Sensors & Peripherals

### 🎛️ Onboard Sensors

- **Accelerometer (3-axis)**  
  Measures acceleration, tilt, shake, tap.  
  _Chip: Bosch BMI270_

- **Magnetometer / Compass (3-axis)**  
  Measures magnetic field strength and heading.  
  _Chip: Bosch BMM150_

- **Temperature Sensor**  
  Internal nRF52833 temperature sensor (not ambient-accurate).

- **Microphone (MEMS)**  
  Onboard sound level sensor with processing hardware.

- **Capacitive Touch Logo**  
  The front gold logo acts as a touch input.

### 🔌 Onboard Peripherals

**Datasheet**: [link](https://raspberrypi.dk/wp-content/uploads/2020/10/BBC-microbit-v2-datasheet-v1.2.pdf)

- **5×5 LED Matrix Display**  
  25 individually addressable LEDs.

- **Buttons A & B**  
  Two tactile user buttons with press, release, and long-press events.

- **Onboard Speaker**  
  Integrated speaker for audio playback.

- **Microphone Activity LED**  
  Small indicator LED showing when microphone is active.

- **Bluetooth Low Energy (BLE)**  
  Powered by Nordic nRF52833.

- **USB Interface**  
  For programming, serial communication, and power.

- **Reset Button**  
  Dedicated reset button on the back.

- **Edge Connector (25-pin)**  
  Provides GPIO, analog inputs, PWM, I2C, SPI, and power pins.

### 🧩 Onboard System Features

- **Improved Power Management**  
  Includes current sensing and efficient regulator.

- **I2C Bus (Shared Internal + External)**  
  Internal sensors + external devices via edge connector.

- **UART and SPI Support**  
  Available via programmable pins.

### Projects

- [01 Hello world](./projects/project00_hello_world/README.md): A simple hello world project
- [02 74HC165 Demo](./projects/project01_hc165_game_shield/): Demo program to discover bit mapping for button for elecfreaks retro game console
