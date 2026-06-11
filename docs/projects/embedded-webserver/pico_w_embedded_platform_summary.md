# Pico W Embedded Control Platform Project

## Project Vision

The project evolved from a simple Raspberry Pi Pico W web server idea into a complete embedded systems platform with:

- Web-based control interface
- USB device interface
- Shared transport-independent business logic
- EEPROM-based configuration persistence
- LED matrix animation engine
- 74HC595 LED pattern engine
- Embedded networking
- Browser-based hardware communication

The overall goal is to create a professional-feeling embedded product architecture while learning core embedded systems concepts.

---

# High-Level System Architecture

```text
                Browser UI
                     │
     ┌───────────────┼───────────────┐
     │                               │
 HTTP/WebSocket                 USB HID
     │                               │
     └───────────────┬───────────────┘
                     ▼
              Unified API Layer
                     ▼
              Command Dispatcher
                     ▼
              Service Layer
        ┌────────┬────────┬────────┐
        ▼        ▼        ▼
     EEPROM   Matrix    LED Bar
```

Key architectural idea:

> Business logic should be transport-independent.

This means:
- Web server
- USB device
- Future BLE/UART interfaces

all talk to the same internal services.

---

# Hardware Components

## Core MCU

- Raspberry Pi Pico W

## Peripherals

### 1. I2C EEPROM

Purpose:
- Persistent configuration storage

Stores:
- Wi-Fi credentials
- LED matrix settings
- LED bar patterns
- animation settings
- device state

### 2. I2C LED Matrix (8x8)

Features:
- bitmap editing
- animations
- effects
- brightness control

### 3. 74HC595 LED Bar Driver

Features:
- 1D pattern sequences
- animation playback
- timing control
- enable/disable output

---

# Configuration System

Recommended configuration structure:

```rust
struct Config {
    version: u16,
    wifi: WifiConfig,
    matrix: MatrixConfig,
    ledbar: LedBarConfig,
    checksum: u32,
}
```

Important concepts:
- versioning
- checksum/CRC validation
- corruption recovery
- default fallback config

---

# LED Matrix System

## UI Features

- 8x8 bitmap editor
- animation selection
- brightness slider
- power toggle

## Suggested Architecture

Separate:
- Pattern
- Animation
- Renderer

### Pattern

Stores bitmap data.

### Animation

Examples:
- blink
- fade
- pulse
- scroll
- wipe

### Renderer

Responsible for:
- timing
- transitions
- frame updates

---

# 74HC595 LED Bar System

## Features

- sequence editor
- frame delay control
- playback
- enable/disable

## Recommended Model

```rust
struct Frame {
    pattern: u8,
    duration_ms: u16,
}
```

Animation timeline:

```rust
Vec<Frame>
```

This becomes a reusable animation engine.

---

# Web Server Features

## AP Mode Setup Flow

On first boot:

```text
Pico-Control-XXXX
```

User:
- connects via phone/laptop
- opens setup page
- enters Wi-Fi credentials

Device:
- saves config to EEPROM
- restarts
- joins Wi-Fi network

---

# Wi-Fi Mode

Once connected:

- starts HTTP server
- serves frontend UI
- exposes REST/WebSocket APIs

Suggested access:

```text
http://pico-control.local
```

using mDNS.

---

# Suggested Web Features

## Dashboard

Display:
- Wi-Fi status
- IP address
- uptime
- memory usage
- firmware version

## LED Indicators

Suggested behavior:

| State | LED |
|---|---|
| AP mode | blinking |
| Connected | solid |
| Error | fast blink |

---

# Recovery Features

## Factory Reset

Possible implementations:
- long button press
- reset sequence

Actions:
- clear EEPROM config
- return to AP mode

## Restart Option

Available from:
- AP mode UI
- normal web UI

---

# Web Frontend Architecture

Recommended stack:

| Component | Recommendation |
|---|---|
| Frontend | Preact |
| Build Tool | Vite |
| CSS | Pico.css |

Alternative:
- pure HTML/CSS/JS

---

# Frontend Delivery Model

The Pico W serves:
- HTML
- CSS
- JavaScript

Browser performs:
- rendering
- UI updates
- fetch/WebSocket communication

The Pico only:
- serves static files
- handles API requests
- controls hardware

---

# HTTP API Design

Suggested REST endpoints:

```text
GET    /api/status
GET    /api/patterns
POST   /api/matrix
PUT    /api/config
POST   /api/restart
POST   /api/reset
```

---

# WebSocket Support

Recommended for:
- live updates
- real-time LED control
- status notifications

Benefits:
- lower latency
- reduced polling
- better UX

---

# USB Device Architecture

## Recommended USB Strategy

Preferred:
- HID device

Avoid initially:
- fully custom USB class

Why HID:
- no kernel driver required
- supported by browsers
- cross-platform
- easy userspace access

---

# WebHID Architecture

Possible architecture:

```text
Browser UI
    ⇅ WebHID
USB HID Device (Pico)
    ⇅
Drivers / Services / Hardware
```

No backend server required.

---

# Why WebHID Is Interesting

The browser can directly communicate with the Pico over USB.

Architecture:

```text
Browser JS
   ↓
WebHID API
   ↓
OS Generic HID Driver
   ↓
Pico HID Device
```

Advantages:
- no Linux daemon required
- no custom kernel driver
- direct browser control

---

# Shared Transport Layer

Recommended frontend abstraction:

```text
Frontend
   ├── HttpTransport
   └── HidTransport
```

Both expose:
- identical commands
- identical responses

UI becomes transport-independent.

---

# Unified Command Protocol

Very important recommendation:

Use same logical protocol for:
- HTTP
- USB HID
- future UART/BLE

Example:

```json
{
  "cmd": "matrix.set_pattern",
  "data": {
    "pattern": [0,24,60,126]
  }
}
```

Benefits:
- reusable frontend
- reusable backend
- scalable architecture

---

# Suggested State Machine

```text
BOOT
 ↓
LOAD_CONFIG
 ↓
WIFI_CONNECT
 ↓
RUNNING
 ↓
AP_FALLBACK
```

---

# Professional Features To Add Later

## 1. OTA Firmware Update

Upload firmware from browser.

## 2. Captive Portal

Automatic setup page when joining AP.

## 3. Config Export/Import

JSON backup system.

## 4. Event Logging

Ring-buffer logs:

```text
WiFi connected
EEPROM loaded
Matrix updated
```

## 5. Device Info Endpoint

```text
GET /api/info
```

---

# Recommended Embedded Architecture

```text
Drivers Layer
  ├── EEPROM
  ├── Matrix
  └── 74HC595

Services Layer
  ├── Config
  ├── Animation Engine
  ├── WiFi Manager
  └── USB Manager

API Layer
  ├── HTTP
  ├── WebSocket
  └── HID Protocol

Frontend
```

---

# Key Engineering Concepts Learned

This single project teaches:

- embedded Rust
- async systems
- networking
- USB
- browser hardware APIs
- web frontend architecture
- protocol design
- state machines
- persistence
- transport abstraction
- hardware abstraction
- embedded UX
- IoT architecture

---

# Required Topics and Concepts To Learn

The following list can be used as a roadmap.

---

# Phase 1 — Embedded Rust Foundations

## Topics

- ownership/borrowing
- async Rust
- embedded-hal
- Embassy framework
- GPIO
- timers
- interrupts

## Small Practice Projects

- blink LED
- button interrupt
- async timer demo
- PWM LED fading

---

# Phase 2 — Peripheral Drivers

## Topics

- I2C protocol
- SPI protocol
- shift registers
- driver abstractions
- TDD for embedded drivers

## Small Projects

- EEPROM read/write driver
- 74HC595 driver
- LED matrix driver
- animation engine

---

# Phase 3 — Networking

## Topics

- TCP/IP basics
- HTTP protocol
- REST APIs
- WebSockets
- mDNS
- AP mode vs station mode

## Small Projects

- simple HTTP server
- REST endpoint
- JSON API
- WebSocket LED control

---

# Phase 4 — Frontend Development

## Topics

- HTML
- CSS
- JavaScript
- fetch API
- WebSocket API
- Preact/Vite
- state management

## Small Projects

- LED control page
- dashboard UI
- bitmap editor
- animation control panel

---

# Phase 5 — Persistent Storage

## Topics

- EEPROM layout
- serialization
- checksums/CRC
- config versioning
- recovery systems

## Small Projects

- save/load settings
- corruption detection
- factory reset implementation

---

# Phase 6 — USB Development

## Topics

- USB fundamentals
- HID descriptors
- endpoints
- report packets
- host/device communication

## Small Projects

- USB HID echo device
- custom HID command protocol
- LED control over USB

---

# Phase 7 — Browser Hardware APIs

## Topics

- WebHID
- browser permissions
- transport abstraction
- frontend device communication

## Small Projects

- browser HID connection
- direct USB LED control
- HID configuration UI

---

# Phase 8 — System Architecture

## Topics

- layered architecture
- command dispatchers
- state machines
- service abstraction
- event systems

## Small Projects

- unified command protocol
- transport-independent API
- reusable animation engine

---

# Recommended Development Strategy

Do NOT build everything at once.

Instead:

## Step 1

Peripheral drivers only.

## Step 2

EEPROM config system.

## Step 3

Animation engines.

## Step 4

HTTP server.

## Step 5

Frontend UI.

## Step 6

USB HID support.

## Step 7

WebHID browser support.

## Step 8

Transport abstraction cleanup.

---

# Final Perspective

This project is no longer just a Raspberry Pi Pico W demo.

It becomes:
- an embedded platform
- a networking project
- a USB device project
- a browser hardware project
- a frontend/backend architecture project
- a systems engineering portfolio project

The most important architectural idea throughout the discussion is:

> Separate transports from business logic.

That single design choice makes the entire system scalable, reusable, and professional.
