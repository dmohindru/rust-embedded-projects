# Summary: Making Embedded Rust Library Testable on `std`

Absolutely! That was a long journey, but you’ve essentially built a **fully testable, modular embedded Rust library** while keeping it usable on real hardware. Here is the summary.

---

# 1. Structuring the Library

- **Separate responsibilities**:

  1. **FrameData**: Pure business logic

     - Holds frames, current index, moves index left/right
     - Fully independent of hardware, can run in `std`

  2. **DisplayDriver**: Trait defining how to render a frame

     - Abstracts hardware differences (pins, MAX7219, etc.)

  3. **MicrobitDriver**: Concrete implementation of `DisplayDriver` for micro:bit

     - Knows about rows, columns, active-low logic
     - Uses `Delay` for row timing

  4. **RefreshEngine**: Drives the refresh cycle

     - Calls `driver.render(frame)` repeatedly

---

# 2. Using Traits to Abstract Hardware

```rust
pub trait DisplayDriver {
    fn render(&mut self, frame: &Frame);
}

pub trait PinOutput {
    fn set_high(&mut self);
    fn set_low(&mut self);
}

pub trait Delay {
    fn delay_us(&mut self, us: u32);
}
```

- **Benefits**:

  - Swap real driver with mock for testing
  - Support multiple hardware backends
  - Keeps library hardware-agnostic

---

# 3. Conditional Compilation (`cfg`)

```rust
#[cfg(not(test))]
use embassy_nrf::gpio::Output;

#[cfg(test)]
use embedded_hal::digital::OutputPin;

#[cfg(not(test))]
pub struct EmbassyDelay;

#[cfg(test)]
pub struct MockDelay { pub calls: usize }
```

- Swap implementations depending on target (hardware vs std)

---

# 4. Delay Abstraction

```rust
pub trait Delay {
    fn delay_us(&mut self, us: u32);
}

impl Delay for EmbassyDelay {
    fn delay_us(&mut self, us: u32) {
        embassy_time::block_for(EmbassyDuration::from_micros(us as u64));
    }
}

impl Delay for MockDelay {
    fn delay_us(&mut self, _us: u32) {
        self.calls += 1;
    }
}
```

- Allows testing without real hardware delays
- Can count calls in unit tests

---

# 5. Handling Pins with Traits

- `PinOutput` allows both real pins and mocks
- Wrap arrays as trait objects for the driver

```rust
rows: [&'a mut dyn PinOutput; 5],
cols: [&'a mut dyn PinOutput; 5],
```

---

# 6. MicrobitDriver Logic (Sync Version)

- `render()` steps:

  1. Turn off all rows
  2. Set column pins according to frame
  3. Activate row
  4. Delay 300 µs
  5. Advance row index

- Fully testable with mocks

---

# 7. Testing Techniques

- **Hardware mocking**: `embedded-hal-mock`
- **Delay mocks** to assert calls
- **FrameData** tests independent of hardware
- **Row/column transactions** verified in unit tests

---

# 8. RefreshEngine

- Drives refresh cycle generically

```rust
pub fn tick<D: DisplayDriver>(&mut self, driver: &mut D, frame: &Frame) {
    driver.render(frame);
}
```

- Works with any driver (real or mock)
- Testable with `heapless::Vec` to capture calls

---

# 9. Key Techniques Learned

1. Thin abstractions using traits (`DisplayDriver`, `PinOutput`, `Delay`)
2. Conditional compilation for `std` vs `no_std`
3. Start with sync version for testability
4. Mock external dependencies
5. Keep business logic (`FrameData`) pure
6. RefreshEngine generic over drivers
7. Trait object slices to handle arrays of pins

---

# ✅ Takeaway

- Fully modular and testable embedded library
- Can run tests on PC without hardware
- Easily extendable for different LED matrices
- FrameData is independent of hardware

---

## Research points

- What problem does `async-trait` solves
- why do we need the need to Box, Vec with async-trait
- Why we need an allocator extern crate alloc
- Why we need an allocator implementation crate like use embedded_alloc::Heap;
