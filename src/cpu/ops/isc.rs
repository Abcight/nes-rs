// ISC - INC subtract
// Increase memory by one, then subtract memory from accu-mulator (with borrow)

use super::AddressingMode;
use super::Cpu;

#[allow(dead_code)]
pub const IMOP: u8 = 0xC8;

impl Cpu {
	pub fn isc(&mut self, mode: &AddressingMode) {
		let data = (self.inc_ret(mode) as i8).wrapping_neg().wrapping_sub(1) as u8;
		let sum = self.register_a as u16 + data as u16 + self.status.get_carry() as u16;
		let carry = sum > 0xff;

		self.status.set_carry(carry);
		let result = sum as u8;

		self.status.set_overflow((data ^ result) & (result ^ self.register_a) & 0x80 != 0);

		self.register_a = result;
		self.set_zero_neg_flags(self.register_a);
	}
}