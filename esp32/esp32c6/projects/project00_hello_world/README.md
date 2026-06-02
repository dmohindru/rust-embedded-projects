cargo build --release

cargo espflash flash --release --monitor

probe-rs attach --chip ESP32C6
probe-rs attach --chip ESP32C6 ../../target/riscv32imac-unknown-none-elf/release/project00_hello_world

probe-rs run \
 --chip ESP32C6 \
 ../../target/riscv32imac-unknown-none-elf/release/project00_hello_world
