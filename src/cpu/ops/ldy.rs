// LDY - Load Y Register
// Y,Z,N = M
// Loads a byte of memory into the Y register setting the zero and negative flags as appropriate.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0xA0;

impl Cpu {
	pub fn ldy(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		let value = self.read(addr);

		self.register_y = value;
		self.set_zero_neg_flags(self.register_y);
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_ldy_load_data() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![IMOP, 0x43, 0x00]);
		assert_eq!(cpu.register_y, 0x43);
		assert!(*cpu.status & 0b0000_0010 == 0b00);
		assert!(*cpu.status & 0b1000_0000 == 0);
	}

	#[test]
	fn test_ldy_zero_flag() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![IMOP, 0x00, 0x00]);
		assert!(*cpu.status & 0b0000_0010 == 0b10);
	}
}
