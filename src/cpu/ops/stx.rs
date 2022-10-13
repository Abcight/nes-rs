// STX - Store X Register
// M = X
// Stores the contents of the X register into memory.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0x86;

impl Cpu {
	pub fn stx(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		self.write(addr, self.register_x);
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_stx_store_data() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa2, 0x43, IMOP, 0x00]);
		assert!(cpu.bus.vram_contains(0x43));
	}
}