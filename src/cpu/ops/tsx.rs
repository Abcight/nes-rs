// TSX - Transfer Stack Pointer to X
// X = S
// Copies the current contents of the stack register into the X register and sets the zero and negative flags as appropriate.

use super::AddressingMode;
use super::Cpu;

#[allow(dead_code)]
pub const IMOP: u8 = 0xBA;

impl Cpu {
	pub fn tsx(&mut self, _mode: &AddressingMode) {
		self.register_x = self.stack_pointer;
		self.set_zero_neg_flags(self.register_x);
	}
}