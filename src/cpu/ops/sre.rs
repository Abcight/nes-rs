// SRE
// Shift right one bit in memory, then EOR accumulator with memory.

use super::AddressingMode;
use super::Cpu;

#[allow(dead_code)]
pub const IMOP: u8 = 0x47;

impl Cpu {
	pub fn sre(&mut self, mode: &AddressingMode) {
		let data = self.lsr_m_ext(mode);
		self.register_a ^= data;
		self.set_zero_neg_flags(self.register_a);
	}
}