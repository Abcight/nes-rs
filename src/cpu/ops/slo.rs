// SLO
// Shift left one bit in memory, then OR accumulator with memory.

use super::AddressingMode;
use super::Cpu;

#[allow(dead_code)]
pub const IMOP: u8 = 0x07;

impl Cpu {
	pub fn slo(&mut self, _mode: &AddressingMode) {
		let data = self.asl_ext();
		self.register_a |= data;
		self.set_zero_neg_flags(self.register_a);
	}
}
