// ROR - Rotate Right
// Move each of the bits in either A or M one place to the right. Bit 7 is filled with the current value of the carry flag whilst the old bit 0 becomes the new carry flag value.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0x6A;

pub fn ror_a(cpu: &mut Cpu, _mode: &AddressingMode) {
	let mut data = cpu.register_a;
	let carry = cpu.status.get_carry();
	cpu.status.set_carry(data & 1 == 1);
	data >>= 1;
	if carry {
		data |= 0b1000_0000;
	}
	cpu.register_a = data;
	cpu.set_zero_neg_flags(cpu.register_a);
}

pub fn ror_m(cpu: &mut Cpu, mode: &AddressingMode) {
	let addr = cpu.get_operand_address(mode);
	let mut data = cpu.read(addr);
	let carry = cpu.status.get_carry();
	cpu.status.set_carry(data & 1 == 1);
	data >>= 1;
	if carry {
		data |= 0b1000_0000;
	}
	cpu.write(addr, data);
	cpu.set_zero_neg_flags(data);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	pub fn test_ror() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0b1000_0001, IMOP, 0x00]);
		assert_eq!(cpu.register_a, 64);
		assert!(cpu.status.get_carry())
	}
}