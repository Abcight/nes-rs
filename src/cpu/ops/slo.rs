// SLO
// Shift left one bit in memory, then OR accumulator with memory.

use super::AddressingMode;
use super::Cpu;
use super::asl_ext;

#[allow(dead_code)]
pub const IMOP: u8 = 0x07;

pub fn slo(cpu: &mut Cpu, _mode: &AddressingMode) {
	let data = asl_ext(cpu);
	cpu.register_a |= data;
	cpu.set_zero_neg_flags(cpu.register_a);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_slo() {
		let mut cpu = Cpu::new();
		cpu.memory = [0b1000_0000; 65535];
		cpu.interpret(vec![0xa9, 0b0011_1111, IMOP, 0, 0x00]);
		assert_eq!(cpu.register_a, 126);
	}
}
