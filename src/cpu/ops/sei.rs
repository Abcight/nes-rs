// SEI - Set Interrupt Disable
// I = 1
// Set the interrupt disable flag to one.

use super::AddressingMode;
use super::Cpu;

#[allow(dead_code)]
pub const IMOP: u8 = 0x78;

impl Cpu {
	pub fn sei(&mut self, _mode: &AddressingMode) {
		self.status.set_interrupt(true);
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_sed() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![IMOP, 0x00]);
		assert!(cpu.status.get_interrupt());
	}
}
