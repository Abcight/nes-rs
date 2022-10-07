// SEC - Set Carry Flag
// C = 1
// Set the carry flag to one.

use super::AddressingMode;
use super::Cpu;

#[allow(dead_code)]
pub const IMOP: u8 = 0x38;

impl Cpu {
	pub fn sec(&mut self, _mode: &AddressingMode) {
		self.status.set_carry(true);
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_sec() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![IMOP, 0x00]);
		assert!(cpu.status.get_carry());
	}
}
