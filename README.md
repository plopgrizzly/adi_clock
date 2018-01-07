# Aldaron's Device Interface / Clock (adi_clock 0.3.0)

Aldaron's Device Interface / Clock is a library developed by Plop Grizzly
for interfacing with the CPU's Real-Time-Clock (RTC) and High Precision
Event Timer (HPET). With this, one can time operations, get the time (TODO),
sleep with precision, and animate smoothly.

[Cargo](https://crates.io/crates/adi_clock) /
[Documentation](https://docs.rs/adi_clock)

## Features
**adi_clock**'s current features:
* Get the state of the RTC (Real Time Clock)
* Measure time passed with the HPET (High Precision Event Timer)
* Create smooth animations with `Pulse` (See documentation)

**adi_clock**'s planned features:
* Get the time of day / date

## Support
**adi_clock**'s current support:
* Unix
* Windows

**adi_clock**'s planned support:
* Arduino and Raspberry Pi (no os)

# Contributing

If you'd like to help implement functions for unsupported platforms, fix bugs,
improve the API or improve the Documentation, then contact me at
jeron.lau@plopgrizzly.com. I'll appreciate any help.
