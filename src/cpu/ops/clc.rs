// CLC - Clear Carry Flag
// C = 0

use super::AddressingMode;
use super::Cpu;

#[allow(dead_code)]
pub const IMOP: u8 = 0x18;

pub fn clc(cpu: &mut Cpu, _mode: &AddressingMode) {
	cpu.status.set_carry(false);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_carry_clear() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0xff, 0x0a, IMOP, 0x00]);
		assert!(!cpu.status.get_carry())
	}
}
