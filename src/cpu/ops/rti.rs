// RTI - Return from Interrupt
// The RTI instruction is used at the end of an interrupt processing routine. It pulls the processor flags from the stack followed by the program counter.

use super::AddressingMode;
use super::Cpu;

#[allow(dead_code)]
pub const IMOP: u8 = 0x40;

pub fn rti(cpu: &mut Cpu, _mode: &AddressingMode) {
	*cpu.status = cpu.pop();
	cpu.status.set_break_min(false);
	cpu.status.set_break_max(true);
	cpu.program_counter = cpu.pop_u16();
}