## LED Pattern service

As an LED pattern service, I own

- LED Pattern Engine
- Led Driver (via trait)
- Storage Driver (via trait)

I will provide functionality to

- set led animation pattern
- get current led animation pattern
- cycle to next led animation pattern
- whenever a new led animation pattern is set either by set or cycle command. I will store the active pattern to a storage device
- I will provide a tick function that get the next animation frame from LED Patten Engine and write to Led Driver

I will NOT have following feature

- I will not drive the next animation frame of my own. But the call of tick function will decide how she wants to call either via user button, timer, some event etc.
