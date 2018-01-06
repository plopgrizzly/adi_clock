// Aldaron's Device Interface / Clock
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/pulse.rs

use std::f32;

/// Call these functions on the return value of Clock::since().  Use `rate_spr`
/// to specify how many seconds it takes to cycle through the animation.
pub trait Pulse {
	/// Returns a number between 0-1. This function is used for animations.
	/// It will take rate_spr seconds to go from 0 to 1. 
	fn pulse_half_linear(&self, rate_spr: f32) -> f32;

	/// Returns a number between 0-1. This function is used for animations.
	/// It will take rate_spr seconds to go from 0 to 1 and back to 0.
	fn pulse_full_linear(&self, rate_spr: f32) -> f32;

	/// Returns a number between 0-1. This function is used for animations.
	/// It will take rate_spr seconds to go from 0 to 1 and back to 0. It
	/// uses cosine underneath to make the animation look smooth, by making
	/// the beginning and end of the animation slower than the middle.
	fn pulse_full_smooth(&self, rate_spr: f32) -> f32;

	/// Returns a number between 0-1. This function is used for animations.
	/// It will take rate_spr seconds to go from 0 to 1. It uses cosine
	/// underneath to make the animation look smooth, by making the
	/// beginning and end of the animation slower than the middle.
	fn pulse_half_smooth(&self, rate_spr: f32) -> f32;
}

impl Pulse for f32 {
	fn pulse_half_linear(&self, rate_spr: f32) -> f32 {
		(self % rate_spr) / rate_spr
	}

	fn pulse_full_linear(&self, rate_spr: f32) -> f32 {
		let rtn = (self % rate_spr) / (rate_spr / 2.0);
		if rtn > 1.0 {
			2.0 - rtn
		}else{
			rtn
		}
	}

	fn pulse_full_smooth(&self, rate_spr: f32) -> f32 {
		1.0 - (((self.pulse_full_linear(rate_spr) * f32::consts::PI)
			.cos() + 1.0) / 2.0)
	}

	fn pulse_half_smooth(&self, rate_spr: f32) -> f32 {
		1.0 - (((self.pulse_half_linear(rate_spr) * f32::consts::PI)
			.cos() + 1.0) / 2.0)
	}
}
