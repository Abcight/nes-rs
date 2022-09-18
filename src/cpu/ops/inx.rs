// INX - Increment X Register
// X,Z,N = X + 1
// Adds one to the X register setting the zero and negative flags as appropriate.

use super::AddressingMode;
use super::Cpu;

pub const IMOP: u8 = 0xE8;

pub fn inx(cpu: &mut Cpu, _mode: &AddressingMode) {
	cpu.register_x = cpu.register_x.overflowing_add(1).0;
	cpu.set_zero_neg_flags(cpu.register_x);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_inx_increment() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![IMOP, 0x00]);
		assert_eq!(cpu.register_x, 1)
	}

	#[test]
	fn test_inx_overflow() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0xff, 0xaa, IMOP, IMOP, 0x00]);
		assert_eq!(cpu.register_x, 1)
	}
}
