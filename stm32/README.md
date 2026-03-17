
# STM32F3 Discovery Embedded Rust Projects

This series of projects explores **onboard features of the STM32F3DISCOVERY** board using **Rust + Embassy**.
No external hardware is required — every project uses only onboard peripherals to build progressively more complex systems.

## Learning Focus
- Safe embedded Rust development using Embassy async tasks.
- Practical coverage of all major STM32 peripherals: GPIO, SPI, I²C, UART, ADC, DAC, Timers.
- Test-Driven Development (TDD) for logic and data processing.
- Host–device communication via UART CLI apps.

## Project Sequence
| # | Project | Key Peripherals | Core Concept |
|---|----------|----------------|---------------|
| 1 | Blink Factory | GPIO, Timer | State machine, event handling |
| 2 | Orientation Tracker | SPI, UART | Sensor fusion, async coordination |
| 3 | Compass Logger | I²C, UART | Data normalization, periodic tasks |
| 4 | UART LED Controller | UART, GPIO | Protocol parsing, host-device interaction |
| 5 | Sensor Fusion Dashboard | SPI, I²C, UART | Integration, telemetry, system design |

Each project has a dedicated `README.md` describing its goal, hardware use, and TDD approach.
