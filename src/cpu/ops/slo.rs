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

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_slo() {
		let mut cpu = Cpu::new();
		cpu.memory = [0b1000_0000; 65535];
		cpu.interpret(vec![0xa9, 0b0011_1111, IMOP, 0, 0x00]);
		assert_eq!(cpu.register_a, 126);
	}
}
