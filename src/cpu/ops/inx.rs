// INX - Increment X Register
// X,Z,N = X + 1
// Adds one to the X register setting the zero and negative flags as appropriate.

use super::AddressingMode;
use super::Cpu;

#[allow(dead_code)]
pub const IMOP: u8 = 0xE8;

impl Cpu {
	pub fn inx(&mut self, _mode: &AddressingMode) {
		self.register_x = self.register_x.overflowing_add(1).0;
		self.set_zero_neg_flags(self.register_x);
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_inx_increment() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![IMOP, 0x00]);
		assert_eq!(cpu.register_x, 1)
	}

	#[test]
	fn test_inx_overflow() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0xff, 0xaa, IMOP, IMOP, 0x00]);
		assert_eq!(cpu.register_x, 1)
	}
}
