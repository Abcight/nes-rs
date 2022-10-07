// PLA - Pull Accumulator
// Pulls an 8 bit value from the stack and into the accumulator. The zero and negative flags are set as appropriate.

use super::AddressingMode;
use super::Cpu;

#[allow(dead_code)]
pub const IMOP: u8 = 0x68;

impl Cpu {
	pub fn pla(&mut self, _mode: &AddressingMode) {
		self.register_a = self.pop();
		self.set_zero_neg_flags(self.register_a);
	}
}