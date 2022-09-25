// SBC - Subtract with Carry
// A,Z,C,N = A-M-(1-C)
// This instruction subtracts the contents of a memory location to the accumulator together with the not of the carry bit. If overflow occurs the carry bit is clear, this enables multiple byte subtraction to be performed.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0xE9;

pub fn sbc(cpu: &mut Cpu, mode: &AddressingMode) {
	let addr = cpu.get_operand_address(mode);
	let data = cpu.read(addr);
	let data = ((data as i8).wrapping_neg().wrapping_sub(1)) as u8;
	let sum = cpu.register_a as u16 + data as u16 + cpu.status.get_carry() as u16;

	let carry = sum > 0xff;
	cpu.status.set_carry(carry);

	let result = sum as u8;
	let overflow = (data ^ result) & (result ^ cpu.register_a) & 0x80 != 0;
	cpu.status.set_overflow(overflow);

	cpu.register_a = result;
	cpu.set_zero_neg_flags(cpu.register_a);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_sbc_overflow() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 1, IMOP, 3, 0x00]);
		assert_eq!(cpu.register_a, 253);
		assert!(!cpu.status.get_carry());
	}

	#[test]
	fn test_sbc_no_overflow() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0xff, IMOP, 3, 0x00]);
		assert_eq!(cpu.register_a, 251);
		assert!(cpu.status.get_carry());
	}
}
