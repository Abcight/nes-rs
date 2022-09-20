// BPL - Branch if Positive
// If the negative flag is clear then add the relative displacement to the program counter to cause a branch to a new location.

use super::AddressingMode;
use super::Cpu;

pub const IMOP: u8 = 0x10;

pub fn bpl(cpu: &mut Cpu, _mode: &AddressingMode) {
	cpu.branch_if(!cpu.status.get_negative());
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_bpl_negative() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0b1000_0000, IMOP, 0x00, 0x00, 0xa9, 4, 0x00]);
		assert_eq!(cpu.register_a, 0b1000_0000)
	}

	#[test]
	fn test_bpl_positive() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0b0000_0000, IMOP, 0x00, 0x00, 0xa9, 4, 0x00]);
		assert_eq!(cpu.register_a, 4)
	}
	
}
