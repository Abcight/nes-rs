// CLV - Clear Overflow Flag
// V = 0
// Clears the overflow flag.

use super::AddressingMode;
use super::Cpu;

pub const IMOP: u8 = 0xB8;

pub fn clv(cpu: &mut Cpu, _mode: &AddressingMode) {
	cpu.status.set_overflow(false);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_overflow_clear() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![IMOP, 0x00]);
		assert!(!cpu.status.get_overflow())
	}
}