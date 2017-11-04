// Aldaron's Device Interface / Clock
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/lib.rs

//! Aldaron's Device Interface / Clock is a library developed by Plop Grizzly
//! for interfacing with the CPU's Real-Time-Clock (RTC) and High Precision
//! Event Timer (HPET). With this, one can time operations, get the time (TODO),
//! sleep with precision, and animate smoothly.

#![warn(missing_docs)]
#![doc(html_logo_url = "http://plopgrizzly.com/adi_clock/icon.png",
	html_favicon_url = "http://plopgrizzly.com/adi_clock/icon.png",
	html_root_url = "http://plopgrizzly.com/adi_clock/")]

mod clock;
mod timer;
mod pulse;

pub use clock::Clock;
pub use pulse::Pulse;
pub use timer::Timer;
