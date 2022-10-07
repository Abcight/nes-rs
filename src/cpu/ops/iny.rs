// INY - Increment Y Register
// Y,Z,N = Y+1
// Adds one to the Y register setting the zero and negative flags as appropriate.

use super::AddressingMode;
use super::Cpu;

#[allow(dead_code)]
pub const IMOP: u8 = 0xC8;

impl Cpu {
	pub fn iny(&mut self, _mode: &AddressingMode) {
		self.register_y = self.register_y.overflowing_add(1).0;
		self.set_zero_neg_flags(self.register_y);
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_iny_increment() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![IMOP, 0x00]);
		assert_eq!(cpu.register_y, 1)
	}

	#[test]
	fn test_iny_overflow() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa0, 0xff, IMOP, 0x00]);
		assert_eq!(cpu.register_y, 0)
	}
}
