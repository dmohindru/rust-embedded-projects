
# Embedded Rust Architecture Guide

## Overview

This document describes a layered architecture for Embedded Rust projects that support:
- Multiple boards (Pico, STM32, ESP32, Micro:bit)
- Multiple transports (HTTP, USB HID, UART, BLE)
- Testable business logic
- Driver abstraction through traits

Core principle:

> Business logic should depend on abstractions (traits), not concrete hardware implementations.

---

# Repository Structure

```text
embedded_core/
microbit/
rpi/
stm32/
esp32/
```

Recommended interpretation:

```text
embedded_core = platform independent code

microbit = firmware and board setup
rpi      = firmware and board setup
stm32    = firmware and board setup
esp32    = firmware and board setup
```

---

# Layered Architecture

```text
HTTP
USB HID
UART
BLE
   │
   ▼
Transport Layer
   │
   ▼
Command Layer
   │
   ▼
Service Layer
   │
   ▼
Driver Traits
   │
   ▼
Driver Implementations
   │
   ▼
Hardware
```

---

# embedded_core Layout

```text
embedded_core/
│
├── drivers/
├── services/
├── commands/
├── models/
├── transport/
└── tests/
```

---

# Models Layer

Purpose:
- Shared data structures
- Configuration objects
- Domain types

Example:

```rust
pub struct Config {
    pub wifi_ssid: String,
    pub brightness: u8,
}
```

Models contain data only.

---

# Drivers Layer

Purpose:
- Direct hardware interaction
- Protocol implementation
- Device initialization

Examples:
- HT16K33
- 74HC595
- EEPROM

Example:

```rust
pub struct Ht16k33<I2C> {
    i2c: I2C,
}
```

Drivers should not contain business logic.

---

# Driver Traits

Purpose:
- Abstract capabilities
- Enable mocking
- Allow service portability

Example:

```rust
pub trait MatrixDisplay {
    fn write_bitmap(
        &mut self,
        bitmap: &[u8; 8],
    );
}
```

Implementation:

```rust
impl<I2C> MatrixDisplay for Ht16k33<I2C> {
    // implementation
}
```

---

# Service Layer

Purpose:
- Business logic
- Application rules
- Device behavior

Examples:
- save configuration
- show startup animation
- play LED sequence

Example:

```rust
pub struct MatrixService<M>
where
    M: MatrixDisplay,
{
    matrix: M,
}
```

Services know nothing about:
- HTTP
- USB
- WiFi
- I2C

---

# Why Services Use Traits

Without traits:

```text
Service
  ↓
HT16K33
  ↓
I2C
```

Hard to test.

With traits:

```text
Service
  ↓
MatrixDisplay Trait
```

Mock implementation:

```rust
struct MockMatrix {
    last_bitmap: [u8; 8],
}
```

Enables TDD and desktop testing.

---

# Commands Layer

Purpose:
- Represent user intentions
- Transport-independent requests

Example:

```rust
pub enum Command {
    MatrixOff,
    SaveConfig,
    MatrixSetPattern {
        pattern: [u8; 8],
    },
}
```

---

# Responses

Purpose:
- Standardized results

Example:

```rust
pub enum Response {
    Ok,
    Error,
    ConfigSaved,
}
```

---

# Dispatcher

Purpose:
- Convert commands into service calls

Example:

```rust
match cmd {
    Command::MatrixOff => {
        self.matrix.clear();
        Response::Ok
    }

    Command::SaveConfig => {
        self.config.save();
        Response::ConfigSaved
    }
}
```

The dispatcher becomes the center of the application.

---

# Transport Layer

Purpose:
- Parse external protocols
- Create commands
- Encode responses

Examples:
- HTTP
- USB HID
- UART
- BLE

No business logic should live here.

---

# HTTP Example

Incoming request:

```http
POST /matrix/off
```

Converted to:

```rust
Command::MatrixOff
```

Then:

```rust
dispatcher.execute(cmd);
```

---

# USB HID Example

Incoming packet:

```text
SET_PATTERN
```

Converted to:

```rust
Command::MatrixSetPattern
```

Then:

```rust
dispatcher.execute(cmd);
```

Same services.
Same dispatcher.

---

# Firmware Crates

Board-specific crates should contain:

- HAL initialization
- Peripheral setup
- Network setup
- USB setup
- Wiring

Example:

```text
rpi/
│
├── wifi/
├── http/
├── usb/
└── main.rs
```

---

# Dependency Wiring

Example:

```rust
let matrix = Ht16k33::new(i2c);

let service =
    MatrixService::new(matrix);

let dispatcher =
    Dispatcher::new(service);
```

Board crates wire everything together.

---

# Incremental Driver Migration

Existing driver:

```rust
pub struct Hc595<SPI> {
    // fields
}
```

Existing code:

```rust
leds.write_byte(0xAA);
```

Keep it unchanged.

Add trait:

```rust
pub trait LedBar {
    fn write_pattern(
        &mut self,
        value: u8,
    );
}
```

Implement trait:

```rust
impl<SPI> LedBar for Hc595<SPI> {
    // implementation
}
```

No existing firmware breaks.

---

# Testing Strategy

Service Tests:

```text
Service
  ↓
Mock Driver
```

Driver Tests:

```text
Driver
  ↓
Real Hardware
```

Example:

```rust
struct MockStorage {
    saved: bool,
}
```

Test:

```rust
assert!(storage.saved);
```

---

# What Should Be Abstracted?

Good candidates:

- Displays
- EEPROMs
- Sensors
- Storage
- Networking interfaces

Rule:

> Abstract what may vary.

Examples:

Different displays:

- HT16K33
- MAX7219

Different storage:

- EEPROM
- Internal Flash
- FRAM

Same service code can use all of them.

---

# Recommended Growth Path

1. Keep existing drivers
2. Add traits
3. Add mocks
4. Introduce services
5. Introduce commands
6. Introduce dispatcher
7. Add HTTP transport
8. Add USB HID transport
9. Reuse same services from both transports

---

# Key Takeaway

The stable core of the system should be:

```text
Command
   ↓
Dispatcher
   ↓
Service
   ↓
Trait
```

Everything else (HTTP, USB HID, BLE, UART, WiFi) becomes an adapter around that core.

This keeps the architecture:
- portable
- testable
- maintainable
- scalable
