// BEQ - Branch if Equal
// If the zero flag is set then add the relative displacement to the program counter to cause a branch to a new location.

use super::AddressingMode;
use super::Cpu;

#[allow(dead_code)]
pub const IMOP: u8 = 0xF0;

impl Cpu {
	pub fn beq(&mut self, _mode: &AddressingMode) {
		self.branch_if(self.status.get_zero());
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_beq_zero() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0, IMOP, 0x00, 0x00, 0xa9, 4, 0x00]);
		assert_eq!(cpu.register_a, 4)
	}

	#[test]
	fn test_beq_not_zero() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 1, IMOP, 0x00, 0x00, 0xa9, 4, 0x00]);
		assert_eq!(cpu.register_a, 1)
	}
}
