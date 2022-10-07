// PLP - Pull Processor Status
// Pulls an 8 bit value from the stack and into the processor flags. The flags will take on new states as determined by the value pulled.

use super::AddressingMode;
use super::Cpu;

#[allow(dead_code)]
pub const IMOP: u8 = 0x28;

impl Cpu {
	pub fn plp(&mut self, _mode: &AddressingMode) {
		*self.status = self.pop();
		self.status.set_break(true);
	}
}