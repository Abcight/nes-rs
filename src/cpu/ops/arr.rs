// ARR - AND byte with accumulator
// AND byte with accumulator, then rotate one bit right in accumulator and check bit 5 and 6.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0x6B;

impl Cpu {
	pub fn arr(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		let data = self.read(addr);

		self.register_a &= data;
		self.set_zero_neg_flags(self.register_a);

		self.ror_a(mode);

		let result = self.register_a;
		let b5 = (result >> 5) & 1;
		let b6 = (result >> 6) & 1;
		self.status.set_carry(b5 == 1);
		self.status.set_overflow(b5 ^ b6 == 1);
		self.set_zero_neg_flags(result);
	}
}