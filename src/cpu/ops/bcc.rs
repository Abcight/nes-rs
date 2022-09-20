// BCC - Branch if Carry Clear
// If the carry flag is clear then add the relative displacement to the program counter to cause a branch to a new location.

use super::AddressingMode;
use super::Cpu;

pub const IMOP: u8 = 0x90;

pub fn bcc(cpu: &mut Cpu, _mode: &AddressingMode) {
	cpu.branch_if(!cpu.status.get_carry());
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_bcc_clear() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0x90, 0x00, 0x00, 0xa9, 4, 0x00]);
		assert_eq!(cpu.register_a, 4)
	}

	#[test]
	fn test_bcc_set() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0x90, 0x00, 0x00, 0xa9, 4, 0x00]);
		assert_eq!(cpu.register_a, 4)
	}
}
