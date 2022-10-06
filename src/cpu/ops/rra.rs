// RRA
// Rotate one bit right in memory, then add memory to accumulator (with carry).

use super::AddressingMode;
use super::Cpu;

#[allow(dead_code)]
pub const IMOP: u8 = 0x27;

impl Cpu {
	pub fn rra(&mut self, mode: &AddressingMode) {
		let data = self.ror_m_ext(mode);
		self.add_a_carry(data);
	}
}