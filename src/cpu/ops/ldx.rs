// LDX - Load X Register
// X,Z,N = M
// Loads a byte of memory into the X register setting the zero and negative flags as appropriate.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

pub const IMOP: u8 = 0xa2;

pub fn ldx(cpu: &mut Cpu, mode: &AddressingMode) {
	let addr = cpu.get_operand_address(mode);
	let value = cpu.read(addr);

	cpu.register_x = value;
	cpu.set_zero_neg_flags(cpu.register_x);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_ldx_load_data() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![IMOP, 0x43, 0x00]);
		assert_eq!(cpu.register_x, 0x43);
		assert!(*cpu.status & 0b0000_0010 == 0b00);
		assert!(*cpu.status & 0b1000_0000 == 0);
	}

	#[test]
	fn test_ldx_zero_flag() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![IMOP, 0x00, 0x00]);
		assert!(*cpu.status & 0b0000_0010 == 0b10);
	}
}
