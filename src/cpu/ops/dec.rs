// DEC - Decrement Memory
// M,Z,N = M-1
// Subtracts one from the value held at a specified memory location setting the zero and negative flags as appropriate.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0xC6;

impl Cpu {
	pub fn dec(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		let mut data = self.read(addr);
		data = data.wrapping_sub(1);
		self.write(addr, data);
		self.set_zero_neg_flags(data);
	}
}