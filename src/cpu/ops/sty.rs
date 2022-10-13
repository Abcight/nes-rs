// STY - Store Y Register
// M = Y
// Stores the contents of the Y register into memory.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0x84;

impl Cpu {
	pub fn sty(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		self.write(addr, self.register_y);
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_sty_store_data() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa0, 0x43, IMOP, 0x00]);
		assert!(cpu.bus.vram_contains(0x43));
	}
}