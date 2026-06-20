# T Architecture for Layered Embedded Systems

## Overview

The T Architecture is a mental model developed during the design of an embedded OLED display project.

The key observation is that each service layer performs two responsibilities:

1. Implements an interface (trait) for the layer above.
2. Owns and depends on an interface (trait) for the layer below.

Visually this forms a "T" shape.

```text
Higher Layer
     |
     v
 Interface A
     ^
     |
  Service
     |
     v
 Interface B
Lower Layer
```

Where:

- Interface A is implemented by the service.
- Interface B is owned and consumed by the service.

## Repeating Pattern

This pattern repeats at every level.

```text
StatusService
    ▲ implements Status API
    │
    ▼ owns Display API

DisplayService
    ▲ implements Display API
    │
    ▼ owns DisplayDevice API

SSD1306 Driver
    ▲ implements DisplayDevice API
```

## Relationship to Existing Architecture Styles

### Layered Architecture

The T Architecture is a specialized view of layered architecture where the focus is on contracts between layers rather than concrete implementations.

### Dependency Inversion Principle

The higher-level policy layer owns the abstraction.

Example:

```rust
trait StatusRenderer {
    fn render(...);
}
```

StatusService owns the abstraction while DisplayService implements it.

### Ports and Adapters (Hexagonal Architecture)

A service exposes one port upward and consumes another port downward.

```text
Incoming Port
      |
      v
   Service
      |
      v
Outgoing Port
```

This is conceptually very close to the T Architecture.

### Clean Architecture

Dependencies flow toward abstractions rather than concrete implementations.

## Embedded Example

```text
Application
      |
      v
StatusService
      |
      v
DisplayService
      |
      v
SSD1306 Driver
      |
      v
I2C Driver
```

Each layer owns the interface immediately below it.

## Filesystem Example

```text
Application
      |
      v
FileSystem
      |
      v
BlockDevice
      |
      v
SPI Flash Driver
      |
      v
SPI Driver
```

Again, every layer forms a T.

## Benefits

- High testability
- Clear separation of responsibilities
- Easy mocking
- Easy replacement of implementations
- Supports TDD naturally
- Scales from embedded systems to server applications

## Core Rule

A layer should:

- Implement an abstraction for the layer above.
- Depend only on abstractions of the layer below.
- Avoid direct dependency on concrete implementations.
