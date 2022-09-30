// ROL - Rotate Left
// Move each of the bits in either A or M one place to the left. Bit 0 is filled with the current value of the carry flag whilst the old bit 7 becomes the new carry flag value.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0x2A;

pub fn rol_m_ext(cpu: &mut Cpu, mode: &AddressingMode) -> u8 {
	let addr = cpu.get_operand_address(mode);
	let mut data = cpu.read(addr);
	let carry = cpu.status.get_carry();
	cpu.status.set_carry(data >> 7 == 1);
	data <<= 1;
	if carry {
		data |= 1;
	}
	cpu.write(addr, data);
	cpu.set_zero_neg_flags(data);
	data
}

pub fn rol_a(cpu: &mut Cpu, _mode: &AddressingMode) {
	let mut data = cpu.register_a;
	let carry = cpu.status.get_carry();
	cpu.status.set_carry(data >> 7 == 1);
	data <<= 1;
	if carry {
		data |= 1;
	}
	cpu.register_a = data;
	cpu.set_zero_neg_flags(cpu.register_a);
}

pub fn rol_m(cpu: &mut Cpu, mode: &AddressingMode) {
	rol_m_ext(cpu, mode);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	pub fn test_rol() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0b1000_0001, IMOP, 0x00]);
		assert_eq!(cpu.register_a, 2);
		assert!(cpu.status.get_carry())
	}
}