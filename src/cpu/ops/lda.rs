// LDA - Load Accumulator
// A,Z,N = M
// Loads a byte of memory into the accumulator setting the zero and negative flags as appropriate.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0xa9;

pub fn lda(cpu: &mut Cpu, mode: &AddressingMode) {
	let addr = cpu.get_operand_address(mode);
	let value = cpu.read(addr);

	cpu.register_a = value;
	cpu.set_zero_neg_flags(cpu.register_a);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_lda_load_data() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![IMOP, 0x43, 0x00]);
		assert_eq!(cpu.register_a, 0x43);
		assert!(*cpu.status & 0b0000_0010 == 0b00);
		assert!(*cpu.status & 0b1000_0000 == 0);
	}

	#[test]
	fn test_lda_zero_flag() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![IMOP, 0x00, 0x00]);
		assert!(*cpu.status & 0b0000_0010 == 0b10);
	}
}
