// ORA - Logical Inclusive OR
// A,Z,N = A|M
// An inclusive OR is performed, bit by bit, on the accumulator contents using the contents of a byte of memory.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

pub const IMOP: u8 = 0x09;

pub fn ora(cpu: &mut Cpu, mode: &AddressingMode) {
	let addr = cpu.get_operand_address(mode);
	let data = cpu.read(addr);
	cpu.register_a |= data;
	cpu.set_zero_neg_flags(cpu.register_a);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_ora() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0x08, IMOP, 0x07, 0x00]);
		assert_eq!(cpu.register_a, 15);
	}
}
