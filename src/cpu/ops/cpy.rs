// CPY - Compare Y Register
// Z,C,N = Y-M
// This instruction compares the contents of the Y register with another memory held value and sets the zero and carry flags as appropriate.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0xC0;

pub fn cpy(cpu: &mut Cpu, mode: &AddressingMode) {
	let addr = cpu.get_operand_address(mode);
	let data = cpu.read(addr);
	cpu.status.set_carry(cpu.register_y >= data);
	cpu.status.set_zero(cpu.register_y == data);
	cpu.set_zero_neg_flags(cpu.register_y.wrapping_sub(data));
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_cpy_less() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xA0, 5, IMOP, 3, 0x00]);
		assert!(cpu.status.get_carry());
		assert!(!cpu.status.get_zero());
	}

	#[test]
	fn test_cpy_equal() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xA0, 5, IMOP, 5, 0x00]);
		assert!(cpu.status.get_carry());
		assert!(cpu.status.get_zero());
	}
}
