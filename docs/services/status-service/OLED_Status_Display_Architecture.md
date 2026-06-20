# OLED Status Display Architecture (Implementation Baseline)

## Goal

Build a layered OLED display system suitable for:

- TDD
- Embedded Rust
- SSD1306 OLED displays
- Future ST7789/TFT support
- Future command dispatcher integration

---

# Status Domain

## Device State

```rust
pub enum DeviceState {
    Booting,
    Ready,
    Measuring,
    Error,
}
```

## Status Snapshot

```rust
pub struct StatusSnapshot {
    pub state: DeviceState,
    pub battery_percent: u8,
    pub uptime_secs: u32,
}
```

---

# Status Renderer Trait

Owned by StatusService.

```rust
pub trait StatusRenderer {
    type Error;

    fn render(
        &mut self,
        snapshot: &StatusSnapshot,
    ) -> Result<(), Self::Error>;

    fn log(
        &mut self,
        message: &str,
    ) -> Result<(), Self::Error>;
}
```

DisplayService will implement this trait.

---

# Status Trait

Used by future services such as CommandService.

```rust
pub trait Status {
    fn set_state(&mut self, state: DeviceState);

    fn set_battery_percent(&mut self, percent: u8);

    fn tick(&mut self);

    fn snapshot(&self) -> StatusSnapshot;

    fn refresh(&mut self);
}
```

---

# Display Commands

Public language of the display subsystem.

```rust
pub enum DisplayCommand<'a> {
    Clear,

    Text {
        row: u8,
        text: &'a str,
    },

    Log {
        text: &'a str,
    },
}
```

---

# Display Service Trait

Implemented by OLED, TFT, console, or other display services.

```rust
pub trait DisplayService {
    type Error;

    fn submit(
        &mut self,
        cmd: DisplayCommand<'_>,
    ) -> Result<(), Self::Error>;

    fn flush(&mut self)
        -> Result<(), Self::Error>;
}
```

Responsibilities:

- Accept display commands
- Manage layout/model
- Convert commands into device operations
- Manage frame lifecycle

---

# Display Device Trait

Hardware abstraction.

Implemented by SSD1306, ST7789, etc.

```rust
pub trait DisplayDevice {
    type Error;

    fn execute(
        &mut self,
        cmd: DisplayCommand<'_>,
    ) -> Result<(), Self::Error>;
}
```

Alternative names:

- Screen
- DisplayTarget
- DisplayDevice

---

# Layering

```text
StatusService
      |
      v
StatusRenderer
      ^
      |
DisplayService
      |
      v
DisplayDevice
      ^
      |
SSD1306
ST7789
Console
```

---

# SSD1306 Implementation Direction

```text
DisplayCommand
       |
       v
Display Model
       |
       v
embedded-graphics
       |
       v
Framebuffer
       |
       v
SSD1306 Driver
       |
       v
I2C
```

---

# Initial OLED Screen

```text
State: READY

Battery: 82%

Up: 01:23:45
```

---

# Immediate Next Implementation Tasks

1. Create StatusSnapshot and DeviceState.
2. Create StatusRenderer trait.
3. Create DisplayCommand enum.
4. Create DisplayService trait.
5. Create DisplayDevice trait.
6. Implement mock DisplayDevice.
7. Implement mock StatusRenderer.
8. Write StatusService TDD tests.
9. Write SSD1306 driver.
10. Integrate embedded-graphics.
