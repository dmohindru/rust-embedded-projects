# Storage Stack Architecture and Testing Strategy

## High Level Architecture

```text
Application
    |
    v
Filesystem
    |
    v
Block Device Interface
    |
    +--------------------+-------------------+-------------------+
    |                    |                   |
    v                    v                   v
SPI Flash Driver    EEPROM Driver      SD Card Driver
    |                    |                   |
    v                    v                   v
SPI HAL            I2C HAL            SDIO/SPI HAL
    |
    v
MCU Peripheral
```

---

# Layer Responsibilities

## 1. Filesystem

Pure software.

Knows:

- Files
- Directories
- Allocation
- Metadata

Does NOT know:

- SPI
- I2C
- SDIO
- STM32

Example:

```rust
fs.read_file("config.txt")
```

---

## 2. Block Device Interface

Provides common abstraction.

Example:

```rust
trait BlockDevice {
    fn read_block(...);
    fn write_block(...);
}
```

Filesystem depends only on this trait.

---

## 3. Device Drivers

Translate block operations into device-specific commands.

Examples:

### SPI Flash

```text
read_block(10)
-> flash.read(address)
```

May need:

- Sector erase
- Page programming
- Busy polling

### EEPROM

```text
read_block(10)
-> multiple I2C transactions
```

### SD Card

```text
read_block(10)
-> sector read command
```

Often maps naturally.

---

## 4. HAL Layer

Provided by Embassy/HAL.

Examples:

```rust
Spi
I2c
Sdio
```

Responsible for peripheral access.

---

# Why Block Devices Exist

Filesystem wants:

```text
Block 0
Block 1
Block 2
```

Not:

```text
SPI transactions
I2C commands
GPIO toggling
```

Block interface hides hardware differences.

---

# Testing Strategy

## Phase 1 - Filesystem Testing

Use RAM disk.

```rust
struct RamDisk {
    storage: [u8; SIZE]
}
```

Benefits:

- Fast
- Deterministic
- Runs on PC
- No hardware needed

Test:

- Create file
- Read file
- Delete file
- List files

---

## Phase 2 - Block Device Testing

Verify:

```text
write block
read block
compare
```

Use property-style tests.

---

## Phase 3 - Device Driver Testing

SPI Flash:

- Read JEDEC ID
- Erase sector
- Write page
- Read back

EEPROM:

- Write page
- Read back

SD Card:

- Read sector
- Write sector
- Read sector

---

## Phase 4 - Integration Testing

Run unchanged filesystem on:

1. RAM disk
2. SPI Flash
3. EEPROM
4. SD Card

Expected:

```text
Same API
Same tests
Different hardware
```

---

# Suggested Repository Layout

```text
embedded-core/
├── storage/
│   ├── block_device.rs
│   ├── ram_disk.rs
│   └── filesystem/
│
├── drivers/
│   ├── spi_flash/
│   ├── eeprom/
│   └── sdcard/
│
stm32/
microbit/
esp32/
```

---

# Main Learning Objective

The filesystem should not care whether the data ultimately lives in:

- RAM
- SPI Flash
- EEPROM
- SD Card

Changing hardware should only require replacing the block-device implementation.
