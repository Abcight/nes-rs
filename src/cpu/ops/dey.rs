// DEY - Decrement Y Register
// Y,Z,N = Y-1
// Subtracts one from the Y register setting the zero and negative flags as appropriate.

use super::AddressingMode;
use super::Cpu;

#[allow(dead_code)]
pub const IMOP: u8 = 0x88;

pub fn dey(cpu: &mut Cpu, _mode: &AddressingMode) {
	cpu.register_y = cpu.register_y.overflowing_sub(1).0;
	cpu.set_zero_neg_flags(cpu.register_y);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_dey_decrement() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xA0, 10, IMOP, 0x00]);
		assert_eq!(cpu.register_y, 9)
	}
}
