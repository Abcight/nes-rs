// LAR
// AND memory with stack pointer, transfer result to accumulator, X register and stack pointer.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0xBB;

impl Cpu {
	pub fn lar(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		let data = self.read(addr) & self.stack_pointer;
		self.register_a = data;
		self.register_x = data;
		self.stack_pointer = data;
		self.set_zero_neg_flags(data);
	}
}