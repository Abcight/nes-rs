// Abc: This is unused on NES, but I have implemented it anyway because it's in the spec.
// CLD - Clear Decimal Mode
// D = 0
use super::AddressingMode;
use super::Cpu;

#[allow(dead_code)]
pub const IMOP: u8 = 0xD8;

impl Cpu {
	pub fn cld(&mut self, _mode: &AddressingMode) {
		self.status.set_decimal(false);
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_decimal_clear() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![IMOP, 0x00]);
		assert!(!cpu.status.get_decimal())
	}
}
