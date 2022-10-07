// CLV - Clear Overflow Flag
// V = 0
// Clears the overflow flag.

use super::AddressingMode;
use super::Cpu;

#[allow(dead_code)]
pub const IMOP: u8 = 0xB8;

impl Cpu {
	pub fn clv(&mut self, _mode: &AddressingMode) {
		self.status.set_overflow(false);
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_overflow_clear() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![IMOP, 0x00]);
		assert!(!cpu.status.get_overflow())
	}
}