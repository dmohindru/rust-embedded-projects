cargo build --release

cargo espflash flash --release --monitor

probe-rs attach --chip ESP32C6
probe-rs attach --chip ESP32C6 target/riscv32imac-unknown-none-elf/release/hello-esp32

// for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.1.0/examples

goals

- logging
- debugger

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "probe-rs-debug",
      "request": "launch",
      "name": "Debug ESP32-C6",
      "chip": "ESP32C6",
      "cwd": "${workspaceFolder}",
      "binary": "${workspaceFolder}/target/riscv32imac-unknown-none-elf/debug/hello-esp32"
    }
  ]
}
```
