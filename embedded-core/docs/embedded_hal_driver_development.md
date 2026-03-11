# Embedded HAL Driver Development Approach (Rust + Embassy)

This document describes the approach used to build a reusable hardware
driver using the `embedded-hal` ecosystem and then integrate it with
concrete hardware using the Embassy async framework.

The goal is to keep the **driver hardware-agnostic** while allowing it
to run on specific microcontrollers.

------------------------------------------------------------------------

# 1. Design Philosophy

The driver is written against **traits**, not concrete hardware
implementations.

Driver\
↓\
embedded-hal traits\
↓\
HAL implementation\
↓\
MCU peripheral

This provides:

-   portability across microcontrollers
-   testability without hardware
-   separation between driver logic and hardware setup

------------------------------------------------------------------------

# 2. Driver Layer (Hardware Agnostic)

The driver depends only on `embedded-hal` traits.

Example:

``` rust
use embedded_hal_async::spi::SpiDevice;

pub struct Max7219Driver<D, const R: usize, const C: usize>
where
    D: SpiDevice,
{
    spi: D,
}
```

Key ideas:

-   The driver **does not know what MCU is used**
-   It only requires a device implementing `SpiDevice`
-   `const generics` allow compile-time configuration (rows/columns)

Advantages:

-   works with any HAL implementing `embedded-hal`
-   unit tests can use mock implementations

------------------------------------------------------------------------

# 3. Hardware Abstraction Layer (HAL)

The HAL provides implementations of `embedded-hal` traits for a specific
microcontroller.

Example HAL:

-   embassy-nrf

This layer exposes peripherals such as:

Spim`<SPI3>`{=html}\
GPIO pins\
interrupts\
DMA

These are still **not directly passed to the driver**.

------------------------------------------------------------------------

# 4. Bus Device Adapter

Most drivers require a **bus device**, not the raw bus.

Example:

SPI bus\
↓\
chip select management\
↓\
SpiDevice wrapper

Embassy provides this adapter:

`embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice`

This wrapper:

-   manages the **CS pin automatically**
-   ensures **exclusive access to the SPI bus**
-   implements `embedded_hal_async::spi::SpiDevice`

------------------------------------------------------------------------

# 5. Shared Bus Infrastructure

Embassy allows multiple devices to share the same SPI bus using a mutex.

Example setup:

``` rust
static SPI_BUS: StaticCell<Mutex<NoopRawMutex, Spim<SPI3>>> = StaticCell::new();

let spi_bus = SPI_BUS.init(Mutex::new(spim));
```

This provides:

-   safe concurrent access
-   async compatibility
-   no data races

------------------------------------------------------------------------

# 6. Concrete Hardware Initialization

Hardware initialization is performed in the application layer.

Example:

``` rust
let spim = spim::Spim::new_txonly(p.SPI3, Irqs, p.P0_13, p.P0_15, config);

let cs = Output::new(
    p.P0_14,
    Level::High,
    OutputDrive::Standard,
);

let spi_device = SpiDevice::new(spi_bus, cs);
```

This step binds:

MCU peripheral\
↓\
SPI bus\
↓\
device wrapper

------------------------------------------------------------------------

# 7. Driver Instantiation

Once the SPI device wrapper is created, it can be passed to the driver.

``` rust
let driver = Max7219::<_, LED_MATRIX_ROWS, LED_MATRIX_COLS>::new(spi_device);
```

The compiler resolves the generic type automatically.

------------------------------------------------------------------------

# 8. Async Task Integration

The driver can then be moved into an async task.

Example:

``` rust
#[embassy_executor::task]
async fn button_receiver(
    receiver: Receiver<'static, ThreadModeRawMutex, Direction, 2>,
    mut max7219: DisplayDriver,
) {
    loop {
        let button_pressed = receiver.receive().await;

        max7219.write_bitmap(...).await.unwrap();
    }
}
```

The task:

-   receives events
-   updates internal state
-   communicates with hardware through the driver

------------------------------------------------------------------------

# 9. Benefits of This Architecture

This layered design provides several advantages.

### Hardware portability

The same driver can run on:

-   nRF52
-   STM32
-   RP2040
-   ESP32

Only the HAL initialization changes.

### Testability

Drivers can be tested using:

-   SPI mocks
-   snapshot tests
-   simulated buffers

### Clean separation of concerns

Application logic\
↓\
Driver\
↓\
embedded-hal traits\
↓\
HAL implementation\
↓\
hardware

------------------------------------------------------------------------

# 10. General Driver Development Workflow

1.  Study the device datasheet
2.  Define driver structure
3.  Implement logic using `embedded-hal` traits
4.  Write unit tests with mock devices
5.  Integrate with HAL
6.  Validate on real hardware

------------------------------------------------------------------------

# 11. Key Embedded Rust Concepts Used

This driver architecture uses several important Rust features:

-   generics
-   const generics
-   trait bounds
-   async/await
-   static memory (`StaticCell`)
-   mutex-based shared bus access

------------------------------------------------------------------------

# 12. Example Stack (Micro:bit v2)

Final software stack used:

Application Task\
↓\
Max7219 Driver\
↓\
embedded-hal async SPI\
↓\
Embassy SPI Device\
↓\
Embassy SPIM driver\
↓\
nRF52833 SPI3 peripheral

------------------------------------------------------------------------

# Conclusion

This approach enables writing **portable, testable, and reusable
embedded drivers** in Rust while maintaining efficient access to
hardware peripherals through HAL implementations.
