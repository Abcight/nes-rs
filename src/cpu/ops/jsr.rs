// JSR - Jump to Subroutine
// The JSR instruction pushes the address (minus one) of the return point on to the stack and then sets the program counter to the target memory address.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

pub const IMOP: u8 = 0x20;

pub fn jsr(cpu: &mut Cpu, _mode: &AddressingMode) {
	cpu.push_u16(cpu.program_counter + 2 - 1);
	cpu.program_counter = cpu.read_u16(cpu.program_counter);
}