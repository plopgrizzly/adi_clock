/**
 * Aldaron's Device Interface - "timer.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the GNU GENERAL PUBLIC LICENSE
**/

extern crate adi_clock;

use adi_clock::Timer;

fn main() {

/*	loop {

	println!("Sleep 1.5 seconds");
	let a = Instant::now();
	Timer::sleep(1.5);
	println!("Slept {} seconds", seconds_since(a));
	
	}*/

	let mut timer = Timer::new(1.0 / 60.0);
	let mut prev = 0.0;

	println!("made Time");

	loop {
		let next = timer.wait();
		println!("{}", next);
		prev = next;
//		println!("waited {}", seconds_since(a));
	}
}
