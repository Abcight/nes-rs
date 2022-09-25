// TSX - Transfer Stack Pointer to X
// X = S
// Copies the current contents of the stack register into the X register and sets the zero and negative flags as appropriate.

use super::AddressingMode;
use super::Cpu;

pub const IMOP: u8 = 0xBA;

pub fn tsx(cpu: &mut Cpu, _mode: &AddressingMode) {
	cpu.register_x = cpu.stack_pointer;
	cpu.set_zero_neg_flags(cpu.register_x);
}
