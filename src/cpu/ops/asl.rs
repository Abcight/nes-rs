// ASL - Arithmetic Shift Left
// A,Z,C,N = M*2 or M,Z,C,N = M*2
// This operation shifts all the bits of the accumulator or memory contents one bit left. Bit 0 is set to 0 and bit 7 is placed in the carry flag. The effect of this operation is to multiply the memory contents by 2 (ignoring 2's complement considerations), setting the carry if the result will not fit in 8 bits.

use super::AddressingMode;
use super::Cpu;

#[allow(dead_code)]
pub const IMOP: u8 = 0x0A;

pub fn asl_ext(cpu: &mut Cpu) -> u8 {
	let mut data = cpu.register_a;
	cpu.status.set_carry(data >> 7 == 1);
	data <<= 1;
	cpu.register_a = data;
	cpu.set_zero_neg_flags(cpu.register_a);
	data
}

pub fn asl(cpu: &mut Cpu, _mode: &AddressingMode) {
	asl_ext(cpu);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_asl() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 2, IMOP, 0x00]);
		assert_eq!(cpu.register_a, 4)
	}

	#[test]
	fn test_asl_overflow() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xA9, 0b1000_0001, 0x0A, 0x00]);
		assert_eq!(cpu.register_a, 2);
		assert_eq!(*cpu.status & 0b0000_0001, 1);
	}
}
