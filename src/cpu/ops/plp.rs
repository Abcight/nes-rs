// PLP - Pull Processor Status
// Pulls an 8 bit value from the stack and into the processor flags. The flags will take on new states as determined by the value pulled.

use super::AddressingMode;
use super::Cpu;

pub const IMOP: u8 = 0x28;

pub fn plp(cpu: &mut Cpu, _mode: &AddressingMode) {
	*cpu.status = cpu.pop();
	cpu.status.set_break(true);
}