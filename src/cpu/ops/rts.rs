// RTS - Return from Subroutine
// The RTS instruction is used at the end of a subroutine to return to the calling routine. It pulls the program counter (minus one) from the stack.

use super::AddressingMode;
use super::Cpu;

#[allow(dead_code)]
pub const IMOP: u8 = 0x60;

impl Cpu {
	pub fn rts(&mut self, _mode: &AddressingMode) {
		self.program_counter = self.pop_u16() + 1;
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_rts_empty() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![IMOP, 0x00]);
		assert_eq!(cpu.stack_pointer, 255);
	}
}
