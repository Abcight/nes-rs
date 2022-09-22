// EOR - Exclusive OR
// A,Z,N = A^M
// An exclusive OR is performed, bit by bit, on the accumulator contents using the contents of a byte of memory.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

pub const IMOP: u8 = 0x49;

pub fn eor(cpu: &mut Cpu, mode: &AddressingMode) {
	let addr = cpu.get_operand_address(mode);
	let data = cpu.read(addr);
	cpu.register_a ^= data;
	cpu.set_zero_neg_flags(cpu.register_a);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_eor() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0b0000_0001, IMOP, 0b0000_0011, 0x00]);
		assert_eq!(cpu.register_a, 0b0000_0010);
	}
}
