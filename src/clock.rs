/**
 * adi_clock - Aldaron's Device Interface - Clock - "clock.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use std::time::Instant;
use std::f32;

/// A Real-Time-Clock (RTC) or equivalent device
#[derive(Copy, Clone)]
pub struct Clock {
	time: Instant,
}

impl Clock {
	/// Return a clock that tells the current time.
	pub fn now() -> Clock {
		Clock { time: Instant::now() }
	}

	/// Returns seconds since clock's creation.
	pub fn since(&self) -> f32 {
		let duration = self.time.elapsed();
		let nanos : f32 = duration.subsec_nanos() as f32
			/ 1_000_000_000.0;
		let secs : f32 = duration.as_secs() as f32;
		return secs + nanos;
	}
}

pub trait Pulse {
	fn half_linear_pulse(&self, rate_spr: f32) -> f32;
	fn full_linear_pulse(&self, rate_spr: f32) -> f32;
	fn full_smooth_pulse(&self, rate_spr: f32) -> f32;
	fn half_smooth_pulse(&self, rate_spr: f32) -> f32;
}

impl Pulse for f32 {
	fn half_linear_pulse(&self, rate_spr: f32) -> f32 {
		(self % rate_spr) / rate_spr
	}
	
	fn full_linear_pulse(&self, rate_spr: f32) -> f32 {
		let rtn = (self % rate_spr) / (rate_spr / 2.0);
		if rtn > 1.0 {
			2.0 - rtn
		}else{
			rtn
		}
	}
	
	fn full_smooth_pulse(&self, rate_spr: f32) -> f32 {
		1.0 - (((self.full_linear_pulse(rate_spr) * f32::consts::PI)
			.cos() + 1.0) / 2.0)
	}
	
	fn half_smooth_pulse(&self, rate_spr: f32) -> f32 {
		1.0 - (((self.half_linear_pulse(rate_spr) * f32::consts::PI)
			.cos() + 1.0) / 2.0)
	}
}
