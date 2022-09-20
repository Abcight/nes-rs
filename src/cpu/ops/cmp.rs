// CMP - Compare
// Z,C,N = A-M
// This instruction compares the contents of the accumulator with another memory held value and sets the zero and carry flags as appropriate.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

pub const IMOP: u8 = 0xC9;

pub fn cmp(cpu: &mut Cpu, mode: &AddressingMode) {
	let addr = cpu.get_operand_address(mode);
	let data = cpu.read(addr);
	cpu.status.set_carry(cpu.register_a >= data);
	cpu.status.set_zero(cpu.register_a == data);
	cpu.set_zero_neg_flags(cpu.register_a.wrapping_sub(data));
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_cmp_less() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 5, IMOP, 3, 0x00]);
		assert!(cpu.status.get_carry());
		assert!(!cpu.status.get_zero());
	}

	#[test]
	fn test_cmp_equal() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 5, IMOP, 5, 0x00]);
		assert!(cpu.status.get_carry());
		assert!(cpu.status.get_zero());
	}
}
