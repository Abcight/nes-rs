// JSR - Jump to Subroutine
// The JSR instruction pushes the address (minus one) of the return point on to the stack and then sets the program counter to the target memory address.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0x20;

impl Cpu {
	pub fn jsr(&mut self, _mode: &AddressingMode) {
		self.push_u16(self.program_counter + 2 - 1);
		self.program_counter = self.read_u16(self.program_counter);
	}
}