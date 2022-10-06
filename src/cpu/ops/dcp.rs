// DCP
// Subtract one from memory.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0xC0;

impl Cpu {
	pub fn dcp(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		let mut data = self.read(addr);

		data = data.wrapping_sub(1);
		self.write(addr, data);

		if data <= self.register_a {
			self.status.set_carry(true);
		}

		self.set_zero_neg_flags(self.register_a.wrapping_sub(data));
	}
}