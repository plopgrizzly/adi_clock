/**
 * adi_clock - Aldaron's Device Interface - Clock - "clock.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use std::time::Instant;
use std::f32;

/// Clock represents the state of a Real-Time-Clock (RTC). You can use it to
/// make animations, time operations, or determine the date and time (TODO). 
pub struct Clock {
	time: Instant,
}

impl Clock {
	/// Get the current state of the Real-Time-Clock (RTC).
	pub fn create() -> Clock {
		Clock { time: Instant::now() }
	}

	/// Get the number of seconds since self was initialized (or since get()
	/// was called).
	pub fn since(&self) -> f32 {
		let duration = self.time.elapsed();
		let nanos : f32 = duration.subsec_nanos() as f32
			/ 1_000_000_000.0;
		let secs : f32 = duration.as_secs() as f32;
		return secs + nanos;
	}
}

pub trait Pulse {
	fn pulse_half_linear(&self, rate_spr: f32) -> f32;
	fn pulse_full_linear(&self, rate_spr: f32) -> f32;
	fn pulse_full_smooth(&self, rate_spr: f32) -> f32;
	fn pulse_half_smooth(&self, rate_spr: f32) -> f32;
}

impl Pulse for f32 {
	/// Returns a number between 0-1. This function is used for animations.
	/// It will take rate_spr seconds to go from 0 to 1. 
	fn pulse_half_linear(&self, rate_spr: f32) -> f32 {
		(self % rate_spr) / rate_spr
	}

	/// Returns a number between 0-1. This function is used for animations.
	/// It will take rate_spr seconds to go from 0 to 1 and back to 0.
	fn pulse_full_linear(&self, rate_spr: f32) -> f32 {
		let rtn = (self % rate_spr) / (rate_spr / 2.0);
		if rtn > 1.0 {
			2.0 - rtn
		}else{
			rtn
		}
	}

	/// Returns a number between 0-1. This function is used for animations.
	/// It will take rate_spr seconds to go from 0 to 1 and back to 0. It
	/// uses cosine underneath to make the animation look smooth, by making
	/// the beginning and end of the animation slower than the middle.
	fn pulse_full_smooth(&self, rate_spr: f32) -> f32 {
		1.0 - (((self.pulse_full_linear(rate_spr) * f32::consts::PI)
			.cos() + 1.0) / 2.0)
	}

	/// Returns a number between 0-1. This function is used for animations.
	/// It will take rate_spr seconds to go from 0 to 1. It uses cosine
	/// underneath to make the animation look smooth, by making the
	/// beginning and end of the animation slower than the middle.
	fn pulse_half_smooth(&self, rate_spr: f32) -> f32 {
		1.0 - (((self.pulse_half_linear(rate_spr) * f32::consts::PI)
			.cos() + 1.0) / 2.0)
	}
}
