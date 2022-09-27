// AND byte with accumulator
// Perform a logical AND on byte with accumulator, then transfer accumulator to X register.

use super::AddressingMode;
use super::Cpu;
use super::Memory;
use super::tax;

#[allow(dead_code)]
pub const IMOP: u8 = 0xAB;

pub fn atx(cpu: &mut Cpu, mode: &AddressingMode) {
	let addr = cpu.get_operand_address(mode);
	let data = cpu.read(addr);

	cpu.register_a &= data;
	cpu.set_zero_neg_flags(cpu.register_a);

	tax::tax(cpu, mode);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_atx() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0b1100_1111, IMOP, 0b1001_0010, 0x00]);
		assert_eq!(cpu.register_a, cpu.register_x);
		assert_eq!(cpu.register_x, 0b1000_0010)
	}
}
