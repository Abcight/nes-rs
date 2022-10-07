// AND byte with accumulator
// Perform a logical AND on byte with accumulator, then transfer accumulator to X register.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0xAB;

impl Cpu {
	pub fn atx(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		let data = self.read(addr);

		self.register_a &= data;
		self.set_zero_neg_flags(self.register_a);

		self.tax(mode);
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_atx() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0b1100_1111, IMOP, 0b1001_0010, 0x00]);
		assert_eq!(cpu.register_a, cpu.register_x);
		assert_eq!(cpu.register_x, 0b1000_0010)
	}
}
