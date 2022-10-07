// SED - Set Decimal Flag
// D = 1
// Set the decimal mode flag to one.

use super::AddressingMode;
use super::Cpu;

#[allow(dead_code)]
pub const IMOP: u8 = 0xF8;

impl Cpu {
	pub fn sed(&mut self, _mode: &AddressingMode) {
		self.status.set_decimal(true);
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_sed() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![IMOP, 0x00]);
		assert!(cpu.status.get_decimal());
	}
}
