# Introduction

Root project folder for ESP32 S3 WROOM-1 micro controller

# Setup

1. Install toolchain

```shell
# Install espup
cargo install espup --locked

# Install the Xtensa toolchain for ESP32-S3
espup install --targets esp32s3
```

2. Check installation

```shell
rustup toolchain list

rustc +esp --version

rustc +esp --print target-list | grep xtensa
```

3. Install espflash utility

```shell
cargo install espflash --locked
```

4. Check if board is detected

```shell
espflash board-info
```

5. The board in download mode. Follow this [link](https://wiki.01studio.cc/en/docs/pycontroller/getting_start/firmware/?utm_source=chatgpt.com)

- Basically hold the boot button
- Then restart the board

# Documentation

- **[01Studio pyController](https://github.com/01studio-lab/pyController)**
