## Workspace for micro:bit v2 projects and shared drivers.

### Structure:

- [**drivers/**](./drivers/): hardware driver crates (blocking, HAL-agnostic)
- [**crates/**](./crates/): higher-level helpers and adapters
- [**projects/**](./projects/): microbit binaries that compose drivers and crates

## BBC micro:bit V2 Projects (Embassy + TDD)

These projects explore micro:bit v2 onboard peripherals and async firmware design.

| #   | Project                                                     | Focus                      | Key Hardware                                     | Concepts                                        |
| --- | ----------------------------------------------------------- | -------------------------- | ------------------------------------------------ | ----------------------------------------------- |
| 1   | [Compass + Level Checker](projects/project01_compass_level) | Sensors + State Machine    | Accelerometer, Magnetometer, Buttons, LEDs, UART | async tasks, FSM, UART logging                  |
| 2   | [Dodge Game](projects/project02_dodge_game)                 | Game loop + PWM            | Buttons, LEDs, Speaker, Touch                    | timing, task coordination, sound output         |
| 3   | [Billboard UART](projects/project03_billboard_uart)         | Host–device link over UART | UART, LEDs                                       | protocol design, shared lib, CLI                |
| 4   | [Billboard BLE](projects/project04_billboard_ble)           | Wireless communication     | BLE, LEDs                                        | GATT services, Android integration              |
| 5   | [Env Monitor](projects/project05_env_monitor)               | Sensors + Logging          | Temperature, Microphone, Buttons, Speaker, UART  | multi-mode app, periodic tasks, threshold logic |

**Recommended order:** 1 → 2 → 3 → 4 → 5

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
