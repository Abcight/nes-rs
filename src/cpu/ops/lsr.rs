// LSR - Logical Shift Right
// A,C,Z,N = A/2 or M,C,Z,N = M/2
// Each of the bits in A or M is shift one place to the right. The bit that was in bit 0 is shifted into the carry flag. Bit 7 is set to zero.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0x4A;

impl Cpu {
	pub fn lsr_a(&mut self, _mode: &AddressingMode) {
		let mut data = self.register_a;
		self.status.set_carry(data & 1 == 1);
		data >>= 1;
		self.register_a = data;
		self.set_zero_neg_flags(self.register_a);
	}

	pub fn lsr_m_ext(&mut self, mode: &AddressingMode) -> u8 {
		let addr = self.get_operand_address(mode);
		let mut data = self.read(addr);
		self.status.set_carry(data & 1 == 1);
		data >>= 1;
		self.write(addr, data);
		self.set_zero_neg_flags(data);
		data
	}

	pub fn lsr_m(&mut self, mode: &AddressingMode) {
		self.lsr_m_ext(mode);
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_lsr_a() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 5, IMOP, 0x00]);
		assert_eq!(cpu.register_a, 2);
		assert!(cpu.status.get_carry())
	}
}
