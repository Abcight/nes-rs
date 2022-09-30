// ADC - Add with Carry
// A,Z,C,N = A+M+C
// This instruction adds the contents of a memory location to the accumulator together with the carry bit. If overflow occurs the carry bit is set, this enables multiple byte addition to be performed.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0x69;

pub fn add_a_carry(cpu: &mut Cpu, data: u8) {
	let sum = cpu.register_a as u16 + data as u16 + cpu.status.get_carry() as u16;

	let carry = sum > 0xff;
	cpu.status.set_carry(carry);

	let result = sum as u8;
	let overflow = (data ^ result) & (result ^ cpu.register_a) & 0x80 != 0;
	cpu.status.set_overflow(overflow);

	cpu.register_a = result;
	cpu.set_zero_neg_flags(cpu.register_a);
}

pub fn adc(cpu: &mut Cpu, mode: &AddressingMode) {
	let addr = cpu.get_operand_address(mode);
	let data = cpu.read(addr);
	add_a_carry(cpu, data);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_adc_no_carry() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 1, IMOP, 3, 0x00]);
		assert_eq!(cpu.register_a, 4);
	}

	#[test]
	fn test_adc_carry() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0xff, IMOP, 3, 0x00]);
		assert_eq!(cpu.register_a, 2);
		assert!(cpu.status.get_carry());
	}
}
