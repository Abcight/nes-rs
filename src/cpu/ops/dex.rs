// DEX - Decrement X Register
// X,Z,N = X-1
// Subtracts one from the X register setting the zero and negative flags as appropriate.

use super::AddressingMode;
use super::Cpu;

#[allow(dead_code)]
pub const IMOP: u8 = 0xCA;

pub fn dex(cpu: &mut Cpu, _mode: &AddressingMode) {
	cpu.register_x = cpu.register_x.overflowing_sub(1).0;
	cpu.set_zero_neg_flags(cpu.register_x);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_dex_decrement() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 10, 0xaa, IMOP, 0x00]);
		assert_eq!(cpu.register_x, 9)
	}
}
