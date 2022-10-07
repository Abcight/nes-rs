// DEX - Decrement X Register
// X,Z,N = X-1
// Subtracts one from the X register setting the zero and negative flags as appropriate.

use super::AddressingMode;
use super::Cpu;

#[allow(dead_code)]
pub const IMOP: u8 = 0xCA;

impl Cpu {
	pub fn dex(&mut self, _mode: &AddressingMode) {
		self.register_x = self.register_x.overflowing_sub(1).0;
		self.set_zero_neg_flags(self.register_x);
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_dex_decrement() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 10, 0xaa, IMOP, 0x00]);
		assert_eq!(cpu.register_x, 9)
	}
}
