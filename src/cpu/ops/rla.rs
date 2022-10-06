// RLA
// Rotate one bit left in memory, then AND accumulator with memory.

use super::AddressingMode;
use super::Cpu;

#[allow(dead_code)]
pub const IMOP: u8 = 0x27;

impl Cpu {
	pub fn rla(&mut self, mode: &AddressingMode) {
		let data = self.rol_m_ext(mode);
		self.register_a &= data;
		self.set_zero_neg_flags(self.register_a);
	}
}