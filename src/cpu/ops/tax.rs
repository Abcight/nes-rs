// TAX - Transfer Accumulator to X
// X = A
// Copies the current contents of the accumulator into the X register and sets the zero and negative flags as appropriate.

use super::AddressingMode;
use super::Cpu;

#[allow(dead_code)]
pub const IMOP: u8 = 0xAA;

impl Cpu {
	pub fn tax(&mut self, _mode: &AddressingMode) {
		self.register_x = self.register_a;
		self.set_zero_neg_flags(self.register_x);
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_tax_move_a_to_x() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 69, IMOP, 0x00]);
		assert_eq!(cpu.register_x, 69)
	}
}
