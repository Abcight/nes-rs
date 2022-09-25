// CLI - Clear Interrupt Disable
// I = 0
// Clears the interrupt disable flag allowing normal interrupt requests to be serviced.

use super::AddressingMode;
use super::Cpu;

#[allow(dead_code)]
pub const IMOP: u8 = 0x58;

pub fn cli(cpu: &mut Cpu, _mode: &AddressingMode) {
	cpu.status.set_interrupt(false);
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