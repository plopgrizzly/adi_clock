/**
 * adi_clock - Aldaron's Device Interface - Clock - "lib.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

pub const VERSION : &'static str = "adi_clock 0.2.0";

mod clock;
mod timer;

pub use clock::Clock;
pub use clock::Pulse;
pub use timer::Timer;
