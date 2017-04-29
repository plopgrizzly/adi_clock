/**
 * adi_clock - Aldaron's Device Interface - Clock - "timer.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use clock::Clock;

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "android"))]
mod ffi {
	use Clock;

	type LazyPointer = usize;

	pub struct Timer {
		timeout: f32,
		clock: Clock,
		ticks: f32,
	}

	#[repr(C)]
	#[derive(Copy, Clone)]
	pub struct TimeVal {
		sec: isize,
		usec: isize,
	}

	extern {
		fn nanosleep(req: *const TimeVal, rem: LazyPointer) -> i32;
	}

	impl Timer {
		pub fn create(secs: f32) -> Timer {
			Timer{ timeout:secs, ticks:secs, clock:Clock::create() }
		}

		pub fn wait(&mut self) -> () {
			let passed = self.clock.since();
			let delay = self.ticks - passed;

			if delay > 0.0 {
				let fsec = delay as isize;
				let usec = ((delay % 1.0) * 1_000_000_000.0)
					as isize;
				let timeout = TimeVal { sec: fsec, usec: usec };

				unsafe {
					nanosleep(&timeout, 0);
				}
			}
			self.ticks += self.timeout;
		}
	}
}

#[cfg(target_os = "windows")]
mod ffi {
	type Handle = usize;
	type LazyPointer = usize;
	type Bool = i32;
	type Long = i32;
	type DWord = u32;

	pub struct Timer {
		timer: Handle,
	}

	#[repr(C)]
	pub struct LargeInt {
		a: i64,
		b: i64,
	}
	
	#[link(name = "winmm")]
	#[link(name = "gdi32")]
	extern "system" {
		fn CreateWaitableTimerW(timer_attrib: LazyPointer,
			manual_reset: Bool, name: LazyPointer) -> Handle;
		fn SetWaitableTimer(timer: Handle, due_time: *const LargeInt,
			period: Long, routine: LazyPointer, crarg: LazyPointer,
			resume: Bool) -> Bool;
		fn WaitForSingleObject(timer: Handle, ms: DWord) -> DWord;
		fn CloseHandle(timer: Handle) -> Bool;
	}

	#[cfg(feature = "checks")]
	fn checks(timer: Handle) -> () {
		if timer == 0 {
			panic!("Failed to create Timer.");
		} else {
			println!("Successfully created Timer.");
		}
	}

	#[cfg(not(feature = "checks"))]
	fn checks(_: Handle) {}

	impl Timer {
		pub fn create(secs: f32) -> Timer {
			let timer = unsafe { CreateWaitableTimerW(0, 0, 0) };
			checks(timer);
			let time = (secs * -10000000.0) as i64;
			let millis = (secs * 1000.0) as Long;
			let time = LargeInt { a: time, b: time };
			let r = unsafe { SetWaitableTimer(timer, &time, millis, 0, 0, 0) };
			if r != 1 {
				panic!("Timer: Failed to create.");
			}
			Timer { timer: timer }
		}

		pub fn wait(&self) -> () {
			unsafe { WaitForSingleObject(self.timer, 0xffffffff); }
		}
	}
	
	impl Drop for Timer {
		fn drop(&mut self) {
			unsafe { CloseHandle(self.timer); }
		}
	}
}

pub struct Timer {
	timer: ffi::Timer,
	clock: Clock,
}

impl Timer {
	pub fn create(secs: f32) -> Timer {
		Timer{ timer: ffi::Timer::create(secs), clock: Clock::create() }
	}

	pub fn wait(&mut self) -> f32 {
		self.timer.wait();
		self.clock.since()
	}

	pub fn sleep(secs: f32) -> f32 {
		Timer::create(secs).wait()
	}
}
