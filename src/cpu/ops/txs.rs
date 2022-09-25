// TXS - Transfer X to Stack Pointer
// S = X
// Copies the current contents of the X register into the stack register.

use super::AddressingMode;
use super::Cpu;

#[allow(dead_code)]
pub const IMOP: u8 = 0x9A;

pub fn txs(cpu: &mut Cpu, _mode: &AddressingMode) {
	cpu.stack_pointer = cpu.register_x;
}