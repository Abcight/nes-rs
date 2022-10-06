// INC - Increment Memory
// M,Z,N = M+1
// Adds one to the value held at a specified memory location setting the zero and negative flags as appropriate.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0xE6;

impl Cpu {
	pub fn inc_ret(&mut self, mode: &AddressingMode) -> u8 {
		let addr = self.get_operand_address(mode);
		let mut data = self.read(addr);
		data = data.wrapping_add(1);
		self.write(addr, data);
		self.set_zero_neg_flags(data);
		data
	}

	pub fn inc(&mut self, mode: &AddressingMode) {
		self.inc_ret(mode);
	}
}