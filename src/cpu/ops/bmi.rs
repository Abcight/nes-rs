// BMI - Branch if Minus
// If the negative flag is set then add the relative displacement to the program counter to cause a branch to a new location.

use super::AddressingMode;
use super::Cpu;

#[allow(dead_code)]
pub const IMOP: u8 = 0x30;

impl Cpu {
	pub fn bmi(&mut self, _mode: &AddressingMode) {
		self.branch_if(self.status.get_negative());
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_bmi_negative() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0b1000_0000, IMOP, 0x00, 0x00, 0xa9, 4, 0x00]);
		assert_eq!(cpu.register_a, 4)
	}

	#[test]
	fn test_bmi_positive() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0b0000_0000, IMOP, 0x00, 0x00, 0xa9, 4, 0x00]);
		assert_eq!(cpu.register_a, 0)
	}
	
}
