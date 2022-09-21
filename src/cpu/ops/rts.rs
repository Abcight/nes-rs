// RTS - Return from Subroutine
// The RTS instruction is used at the end of a subroutine to return to the calling routine. It pulls the program counter (minus one) from the stack.

use super::AddressingMode;
use super::Cpu;

pub const IMOP: u8 = 0x60;

pub fn rts(cpu: &mut Cpu, _mode: &AddressingMode) {
	cpu.program_counter = cpu.pop_u16() + 1;
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
