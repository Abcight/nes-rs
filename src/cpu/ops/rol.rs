// ROL - Rotate Left
// Move each of the bits in either A or M one place to the left. Bit 0 is filled with the current value of the carry flag whilst the old bit 7 becomes the new carry flag value.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0x2A;

impl Cpu {
	pub fn rol_m_ext(&mut self, mode: &AddressingMode) -> u8 {
		let addr = self.get_operand_address(mode);
		let mut data = self.read(addr);
		let carry = self.status.get_carry();
		self.status.set_carry(data >> 7 == 1);
		data <<= 1;
		if carry {
			data |= 1;
		}
		self.write(addr, data);
		self.set_zero_neg_flags(data);
		data
	}

	pub fn rol_a(&mut self, _mode: &AddressingMode) {
		let mut data = self.register_a;
		let carry = self.status.get_carry();
		self.status.set_carry(data >> 7 == 1);
		data <<= 1;
		if carry {
			data |= 1;
		}
		self.register_a = data;
		self.set_zero_neg_flags(self.register_a);
	}

	pub fn rol_m(&mut self, mode: &AddressingMode) {
		self.rol_m_ext(mode);
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	pub fn test_rol() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0b1000_0001, IMOP, 0x00]);
		assert_eq!(cpu.register_a, 2);
		assert!(cpu.status.get_carry())
	}
}