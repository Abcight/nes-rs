// SXA
// AND X register with the high byte of the target address of the argument

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0x9E;

impl Cpu {
	pub fn sxa(&mut self, _mode: &AddressingMode) {
		let addr = self.read_u16(self.program_counter) + self.register_y as u16;
		let data = self.register_x & ((addr >> 8) as u8 + 1);
		self.write(addr, data);
	}
}