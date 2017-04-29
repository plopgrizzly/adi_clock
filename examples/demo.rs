extern crate adi_clock;

use adi_clock::{ Clock, Pulse, Timer, VERSION };

fn printout(h: f32) {
	let mut h = (h * 80.0) as isize;
	if h > 79 { h = 79; }
	if h < 0 { h = 0; }

	print!("\r");
	for _ in 0..h {
		print!(".");
	}
	print!("x");
	for _ in (h + 1)..80 {
		print!(".");
	}
	println!();
}

fn main() {
	println!("Demo: {}", VERSION);

	Timer::sleep(1.0);

	let clock = Clock::create();
	let mut timer = Timer::create(1.0 / 60.0);
	let mut tv = 0.0;

	loop {
		printout(tv.pulse_half_linear(10.0));
		printout(tv.pulse_half_smooth(10.0));
		printout(tv.pulse_full_linear(10.0));
		printout(tv.pulse_full_smooth(10.0));
		println!("Seconds elapsed: {}", clock.since());
		tv = timer.wait();
		print!("\x1B[1A\x1B[1A\x1B[1A\x1B[1A\x1B[1A");
	}
}
