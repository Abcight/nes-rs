// PHA - Push Accumulator
// Pushes a copy of the accumulator on to the stack.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0x48;

pub fn pha(cpu: &mut Cpu, _mode: &AddressingMode) {
	cpu.write((crate::cpu::STACK as u16) + cpu.stack_pointer as u16, cpu.register_a);
	cpu.stack_pointer = cpu.stack_pointer.wrapping_sub(1);
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
