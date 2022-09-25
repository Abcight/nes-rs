// SEC - Set Carry Flag
// C = 1
// Set the carry flag to one.

use super::AddressingMode;
use super::Cpu;

pub const IMOP: u8 = 0x38;

pub fn sec(cpu: &mut Cpu, _mode: &AddressingMode) {
	cpu.status.set_carry(true);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_sec() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![IMOP, 0x00]);
		assert!(cpu.status.get_carry());
	}
}
