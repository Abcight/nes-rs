// TXS - Transfer X to Stack Pointer
// S = X
// Copies the current contents of the X register into the stack register.

use super::AddressingMode;
use super::Cpu;

#[allow(dead_code)]
pub const IMOP: u8 = 0x9A;

impl Cpu {
	pub fn txs(&mut self, _mode: &AddressingMode) {
		self.stack_pointer = self.register_x;
	}
}