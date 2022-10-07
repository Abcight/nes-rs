// CLI - Clear Interrupt Disable
// I = 0
// Clears the interrupt disable flag allowing normal interrupt requests to be serviced.

use super::AddressingMode;
use super::Cpu;

#[allow(dead_code)]
pub const IMOP: u8 = 0x58;

impl Cpu {
	pub fn cli(&mut self, _mode: &AddressingMode) {
		self.status.set_interrupt(false);
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_interrupt_clear() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![IMOP, 0x00]);
		assert!(!cpu.status.get_interrupt())
	}
}