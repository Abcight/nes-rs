// BVC - Branch if Overflow Clear
// If the overflow flag is clear then add the relative displacement to the program counter to cause a branch to a new location.

use super::AddressingMode;
use super::Cpu;

pub const IMOP: u8 = 0x50;

pub fn bvc(cpu: &mut Cpu, _mode: &AddressingMode) {
	cpu.branch_if(!cpu.status.get_overflow());
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_bvc_clear() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0, 0x0a, IMOP, 0x00, 0x00, 0xa9, 4, 0x00]);
		assert_eq!(cpu.register_a, 4)
	}

	#[test]
	fn test_bvc_set() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0xff, 0x0a, IMOP, 0x00, 0x00, 0xa9, 4, 0x00]);
		assert_eq!(cpu.register_a, 4)
	}
}
