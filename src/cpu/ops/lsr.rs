// LSR - Logical Shift Right
// A,C,Z,N = A/2 or M,C,Z,N = M/2
// Each of the bits in A or M is shift one place to the right. The bit that was in bit 0 is shifted into the carry flag. Bit 7 is set to zero.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0x4A;

pub fn lsr_a(cpu: &mut Cpu, _mode: &AddressingMode) {
	let mut data = cpu.register_a;
	cpu.status.set_carry(data & 1 == 1);
	data >>= 1;
	cpu.register_a = data;
	cpu.set_zero_neg_flags(cpu.register_a);
}

pub fn lsr_m(cpu: &mut Cpu, mode: &AddressingMode) {
	let addr = cpu.get_operand_address(mode);
	let mut data = cpu.read(addr);
	cpu.status.set_carry(data & 1 == 1);
	data >>= 1;
	cpu.write(addr, data);
	cpu.set_zero_neg_flags(data);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_lsr_a() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 5, IMOP, 0x00]);
		assert_eq!(cpu.register_a, 2);
		assert!(cpu.status.get_carry())
	}
}
