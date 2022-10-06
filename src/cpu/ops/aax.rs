// AAX - AND X register with accumulator
// Store the result in memory.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0x87;

impl Cpu {
	pub fn aax(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		self.write(addr, self.register_a & self.register_x);
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_aax() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0b1000_0001, 0xa2, 0b1000_0100, IMOP, 0x00]);
		assert!(cpu.memory.contains(&0b1000_0000))
	}
}