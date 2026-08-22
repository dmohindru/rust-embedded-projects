### Core philosophy

The input crate should answer:

> What inputs does this device expose, and what are their current values?

It should not answer:

> What should the application do with those inputs?

### Architecture

```text
┌──────────────────────┐
│     Input Device     │
│ Nunchuk / Joystick / │
│ TCA9555 / 74HC195    │
└──────────┬───────────┘
           │ async I/O
           ▼
┌──────────────────────┐
│     InputState       │
│ current input values │
└──────────┬───────────┘
           │ synchronous
           ▼
┌──────────────────────┐
│     Application      │
│ Dashboard / Game     │
└──────────┬───────────┘
           │
           ▼
       DrawTarget
           │
           │ async I/O
           ▼
        Display
```

### Suggested File structure

```text
Embedded Input crate
embedded-input
│
├── InputDevice
├── InputState
├── InputEvent
├── Button
├── Axis
├── Acceleration
├── capabilities/
│ ├── ButtonInput
│ ├── AxisInput
│ └── MotionInput
└── mapper/
└── InputMapper
```

### InputDescriptor

The descriptor is static metadata describing the device's inputs.

Conceptually:

```text
InputDescriptor
│
├── Button
│   ├── C Button
│   └── Z Button
│
├── Axis
│   ├── Joystick X
│   └── Joystick Y
│
└── Motion
    ├── Acceleration X
    ├── Acceleration Y
    └── Acceleration Z
```

### InputState

This is the dynamic counterpart of the descriptor.

The relationship is:

```text
InputDescription                 InputState
─────────────────                ──────────
InputId(0)                       InputId(0)
"Joystick X"       ──────────►   127

InputId(1)                       InputId(1)
"Joystick Y"       ──────────►   82

InputId(2)                       InputId(2)
"C Button"         ──────────►   false

InputId(3)                       InputId(3)
"Z Button"         ──────────►   true
```

So:

> Descriptor tells you what an ID means. State tells you its current value.

### The four-phase validation plan

This is an important part of the design.
**Phase 1 — Nunchuk**

Prove:

```text
Buttons
Axes
Acceleration
```

and build the first Input Dashboard.

**Phase 2 — Analog joystick**

Prove that the abstraction isn't secretly a Nunchuk abstraction.

```text
Axes only
```

**Phase 3 — 74HC195**

Test a fundamentally different input device:

```text
digital inputs
```

**Phase 4 — TCA9555**

Test:

```text
multiple GPIO inputs
```

At every phase ask:

> Did the generic abstraction need to change, or can the new driver simply implement it?

Some changes are expected. That's **part of the plan**, particularly during phases 1–3.

Once we've got all four working, we can consider the abstraction sufficiently exercised to use as the foundation for your games.

### The Input Dashboard becomes our reference application

This is probably the most valuable part of the whole exercise.

```text
Nunchuk ────────┐
Joystick ───────┤
74HC195 ────────┤
TCA9555 ────────┤
                 ▼
          InputDevice
                 │
       ┌─────────┴─────────┐
       ▼                   ▼
InputDescriptor        InputState
       │                   │
       └─────────┬─────────┘
                 ▼
          Input Dashboard
                 │
                 ▼
              Display
```

The dashboard should know nothing about Nunchuks, TCA9555s, etc.

It should simply enumerate the descriptor and retrieve values from the state.

That makes it a really good architectural test.
