# micro:bit v2 Projects (Embassy + TDD)

These projects explore micro:bit v2 onboard peripherals and async firmware design.

| # | Project | Focus | Key Hardware | Concepts |
|---|----------|-------|---------------|-----------|
| 1 | [Compass + Level Checker](project01_compass_level) | Sensors + State Machine | Accelerometer, Magnetometer, Buttons, LEDs, UART | async tasks, FSM, UART logging |
| 2 | [Dodge Game](project02_dodge_game) | Game loop + PWM | Buttons, LEDs, Speaker, Touch | timing, task coordination, sound output |
| 3 | [Billboard UART](project03_billboard_uart) | Host–device link over UART | UART, LEDs | protocol design, shared lib, CLI |
| 4 | [Billboard BLE](project04_billboard_ble) | Wireless communication | BLE, LEDs | GATT services, Android integration |
| 5 | [Env Monitor](project05_env_monitor) | Sensors + Logging | Temperature, Microphone, Buttons, Speaker, UART | multi-mode app, periodic tasks, threshold logic |

**Recommended order:** 1 → 2 → 3 → 4 → 5
