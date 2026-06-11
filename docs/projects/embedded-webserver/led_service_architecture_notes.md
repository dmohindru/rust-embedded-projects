# LED Pattern Engine & Service Architecture Notes

## Goal

Design a reusable LED subsystem that can be:

- Tested independently of hardware
- Reused across multiple hardware implementations
- Integrated later into:
  - Embedded web server
  - USB device interface
  - UART CLI
  - Other command-driven systems

---

# High-Level Architecture

```text
Command Source (HTTP / USB / UART)
            ↓
        Dispatcher
            ↓
        Service
            ↓
       Driver Trait
            ↓
 Concrete Hardware Driver
            ↓
       Physical Device
```

## Pattern Engine

Generates animation frames only.

Examples:

- All blink
- Alternate blink
- Chaser
- Group chaser
- Random blink

Should know nothing about SPI, I2C, GPIO, HC595, HT16K33, or WS2812.

```text
next_frame() -> LedFrame
```

## Frame Representation

```rust
pub struct LedFrame {
    bits: u32,
}
```

Benefits:

- Compact
- Hardware independent
- Easy to test
- Easy to serialize

## Driver Trait

```rust
pub trait LedDriver {
    type Error;

    fn write_frame(
        &mut self,
        frame: &LedFrame,
    ) -> Result<(), Self::Error>;
}
```

**IMPORTANT**: The service defines the trait it needs.

## Hardware Driver

Example:

```rust
pub struct Hc595<SPI, PIN, const N: usize> {
    // hardware specific fields
}
```

Expose low-level capabilities:

```rust
write(...)
enable(...)
disable(...)
clear(...)
```

Keep these close to hardware behaviour.

## Implementing Traits on Existing Drivers

A concrete driver can implement multiple traits.

Example:

```rust
impl<SPI, PIN, const N: usize>
    LedDriver for Hc595<SPI, PIN, N>
{
    fn write_frame(
        &mut self,
        frame: &LedFrame,
    ) -> Result<(), Self::Error> {

        let bytes = frame.bits.to_le_bytes();

        self.write(&bytes)?;

        Ok(())
    }
}
```

**Important insight:**

> Trait implementations can freely call methods already implemented on the struct.

## Service Layer

```rust
pub struct LedService<D>
where
    D: LedDriver,
{
    driver: D,
    engine: PatternEngine,
}
```

Responsibilities:

- Own pattern engine
- Own driver
- Generate frames
- Push frames to hardware
- React to commands

## Testing Strategy

### Pattern Engine Tests

Test pure animation logic.

### Service Tests

Mock the driver trait.

```rust
struct MockLedDriver;
```

No hardware required.

### Dispatcher Tests

Create a service trait.

```rust
pub trait LedService {
    fn set_pattern(...);
    fn tick(...);
}
```

Dispatcher depends on the service trait and can be tested with mocks.

## Important Principle

Good:

```text
LedService
      ↓
 LedDriver Trait
      ↓
     Hc595
```

Bad:

```text
Hc595
   ↓
Business Logic
```

Drivers expose hardware capabilities.

Services contain application behaviour.

## Refined Mental Model

1. Create concrete hardware drivers.
2. Service layer defines the abstraction it needs **from hardware driver**.
3. Hardware drivers implement that abstraction.
4. Services depend on traits.
5. Tests use mocks.

Traits define boundaries.

Hardware adapts itself to those boundaries.

## Final Takeaway

```text
Pattern Engine
      ↓
Service
      ↓
Trait (Port)
      ↓
Hardware Adapter
      ↓
Device
```

Benefits:

- Testable
- Reusable
- Hardware independent
- Web-server friendly
- USB friendly
- Scales to larger firmware projects
