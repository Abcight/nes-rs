// PHA - Push Accumulator
// Pushes a copy of the accumulator on to the stack.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0x48;

impl Cpu {
	pub fn pha(&mut self, _mode: &AddressingMode) {
		self.write((crate::cpu::STACK as u16) + self.stack_pointer as u16, self.register_a);
		self.stack_pointer = self.stack_pointer.wrapping_sub(1);
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_pha() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 16, IMOP, 0x00]);
		assert!(cpu.memory.contains(&16));
	}
}
