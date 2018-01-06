// Aldaron's Device Interface / Clock
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/clock.rs

use std::time::Instant;

/// Clock represents the state of a Real-Time-Clock (RTC). You can use it to
/// make animations, time operations, or determine the date and time (TODO). 
pub struct Clock {
	time: Instant,
}

impl Clock {
	/// Get the current state of the Real-Time-Clock (RTC).
	pub fn new() -> Clock {
		Clock { time: Instant::now() }
	}

	/// Get the number of seconds since self was initialized with
	/// `Clock::new()`.
	pub fn since(&self) -> f32 {
		let duration = self.time.elapsed();
		let nanos : f32 = duration.subsec_nanos() as f32
			/ 1_000_000_000.0;
		let secs : f32 = duration.as_secs() as f32;
		return secs + nanos;
	}
}
