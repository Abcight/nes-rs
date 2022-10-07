// RTI - Return from Interrupt
// The RTI instruction is used at the end of an interrupt processing routine. It pulls the processor flags from the stack followed by the program counter.

use super::AddressingMode;
use super::Cpu;

#[allow(dead_code)]
pub const IMOP: u8 = 0x40;

impl Cpu {
	pub fn rti(&mut self, _mode: &AddressingMode) {
		*self.status = self.pop();
		self.status.set_break_min(false);
		self.status.set_break_max(true);
		self.program_counter = self.pop_u16();
	}
}