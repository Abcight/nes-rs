// ASL - Arithmetic Shift Left
// A,Z,C,N = M*2 or M,Z,C,N = M*2
// This operation shifts all the bits of the accumulator or memory contents one bit left. Bit 0 is set to 0 and bit 7 is placed in the carry flag. The effect of this operation is to multiply the memory contents by 2 (ignoring 2's complement considerations), setting the carry if the result will not fit in 8 bits.

use super::AddressingMode;
use super::Cpu;

pub const IMOP: u8 = 0x0A;

pub fn asl(cpu: &mut Cpu, _mode: &AddressingMode) {
	cpu.register_a = cpu.register_a.rotate_left(1);
	if cpu.register_a & 0b0000_0001 == 1 {
		cpu.status |= 0b0000_0001;
	}
	cpu.register_a &= 0b1111_1110;
	cpu.set_zero_neg_flags(cpu.register_a);
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
		cpu.interpret(vec![0xa9, 0b1000_0001, IMOP, 0x00]);
		assert_eq!(cpu.register_a, 2);
		assert_eq!(cpu.status & 0b0000_0001, 1);
	}
}
