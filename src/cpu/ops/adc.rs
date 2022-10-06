// ADC - Add with Carry
// A,Z,C,N = A+M+C
// This instruction adds the contents of a memory location to the accumulator together with the carry bit. If overflow occurs the carry bit is set, this enables multiple byte addition to be performed.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0x69;

impl Cpu {
	pub fn add_a_carry(&mut self, data: u8) {
		let sum = self.register_a as u16 + data as u16 + self.status.get_carry() as u16;

		let carry = sum > 0xff;
		self.status.set_carry(carry);

		let result = sum as u8;
		let overflow = (data ^ result) & (result ^ self.register_a) & 0x80 != 0;
		self.status.set_overflow(overflow);

		self.register_a = result;
		self.set_zero_neg_flags(self.register_a);
	}

	pub fn adc(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		let data = self.read(addr);
		self.add_a_carry(data);
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_adc_no_carry() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 1, IMOP, 3, 0x00]);
		assert_eq!(cpu.register_a, 4);
	}

	#[test]
	fn test_adc_carry() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0xff, IMOP, 3, 0x00]);
		assert_eq!(cpu.register_a, 2);
		assert!(cpu.status.get_carry());
	}
}
