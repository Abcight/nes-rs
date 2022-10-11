// XAS

use super::AddressingMode;
use super::Cpu;
use super::Memory;

impl Cpu {
	pub fn xas(&mut self, _mode: &AddressingMode) {
		let data = self.register_a & self.register_x;
		self.stack_pointer = data;
		let addr = self.read_u16(self.program_counter) + self.register_y as u16;

		let data = ((addr >> 8) as u8 + 1) & self.stack_pointer;
		self.write(addr, data);
	}
}