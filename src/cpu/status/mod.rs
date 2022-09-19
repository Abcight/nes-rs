use std::ops::Deref;
use std::ops::DerefMut;

// The Obelisk 5069 status is stored in a single 8 bit value
// 7 6 5 4 3 2 1 0
// N V * B D I Z C
pub struct CpuStatus(pub u8);
impl CpuStatus {
	fn set_flag(&mut self, shift: u8, on: bool) {
		if on {
			self.0 |= 0b0000_0001 << shift;
		} else {
			let mut mask = 0b1111_1110u8;
			mask = mask.rotate_left(shift as u32);
			self.0 &= mask;
		}
	}

	pub fn set_negative(&mut self, on: bool) {
		self.set_flag(7, on);
	}

	pub fn set_overflow(&mut self, on: bool) {
		self.set_flag(6, on);
	}

	pub fn set_break(&mut self, on: bool) {
		self.set_flag(4, on);
	}

	pub fn set_decimal(&mut self, on: bool) {
		self.set_flag(3, on);
	}

	pub fn set_interrupt(&mut self, on: bool) {
		self.set_flag(2, on);
	}

	pub fn set_zero(&mut self, on: bool) {
		self.set_flag(1, on);
	}

	pub fn set_carry(&mut self, on: bool) {
		self.set_flag(0, on);
	}
}

impl Deref for CpuStatus {
	type Target = u8;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for CpuStatus {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}