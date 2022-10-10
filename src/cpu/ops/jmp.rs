// JMP - Jump
// Sets the program counter to the address specified by the operand..
// Note: This instruction was buggy on the original hardware. This implementation accounts for that.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0x4C;

impl Cpu {
	pub fn jmp_absolute(cpu: &mut Cpu, _mode: &AddressingMode) {
		cpu.program_counter = cpu.read_u16(cpu.program_counter);
	}

	pub fn jmp(cpu: &mut Cpu, _mode: &AddressingMode) {
		cpu.program_counter = {
			let addr = cpu.read_u16(cpu.program_counter);
			if addr & 0x00FF == 0x00FF {
				let lo = cpu.read(addr);
				let hi = cpu.read(addr & 0xFF00);
				(hi as u16) << 8 | (lo as u16)
			} else {
				cpu.read_u16(addr)
			}
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_jmp() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0x6C, 10, 0x00]);
		assert_eq!(cpu.program_counter, 2);

		cpu.interpret(vec![0x4C, 10, 0x00]);
		assert_eq!(cpu.program_counter, 12);
	}
}
