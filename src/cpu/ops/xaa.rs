// XAA - Undefined Behavior

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0x8B;

impl Cpu {
	// Tried replicating the UB
	pub fn xaa(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		let data = self.read(addr);
		self.register_a = self.register_x;
		self.set_zero_neg_flags(self.register_a);
		self.register_a &= data;
	}
}