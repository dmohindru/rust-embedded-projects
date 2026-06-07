# Introduction

This project serves as the hello world program for esp32 platform

# Links

https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description

# Goals

- what is defmt
- what component forms defmt
- why use defmt over rtt
- what is panic handler for defmt
- how is panic handler different in defmt and rtt
- how is defmt different for different architecture
- how is probe-rs linked to it

espflash flash \
 target/riscv32imac-unknown-none-elf/release/project00_hello_world \
 --monitor \
 --log-format defmt
