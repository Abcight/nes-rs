// PHP - Push Processor Status
// Pushes a copy of the status flags on to the stack.

use super::AddressingMode;
use super::Cpu;

#[allow(dead_code)]
pub const IMOP: u8 = 0x08;

pub fn php(cpu: &mut Cpu, _mode: &AddressingMode) {
	let mut flags = *cpu.status;
	flags |= 0b0011_0000;
	cpu.push(flags);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_php() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 16, IMOP, 0x00]);
		assert!(cpu.memory.contains(&(*cpu.status | 0b0011_0000)))
	}
}
