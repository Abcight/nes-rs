// EOR - Exclusive OR
// A,Z,N = A^M
// An exclusive OR is performed, bit by bit, on the accumulator contents using the contents of a byte of memory.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0x49;

impl Cpu {
	pub fn eor(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		let data = self.read(addr);
		self.register_a ^= data;
		self.set_zero_neg_flags(self.register_a);
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_eor() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0b0000_0001, IMOP, 0b0000_0011, 0x00]);
		assert_eq!(cpu.register_a, 0b0000_0010);
	}
}
