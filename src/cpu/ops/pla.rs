// PLA - Pull Accumulator
// Pulls an 8 bit value from the stack and into the accumulator. The zero and negative flags are set as appropriate.

use super::AddressingMode;
use super::Cpu;

#[allow(dead_code)]
pub const IMOP: u8 = 0x68;

pub fn pla(cpu: &mut Cpu, _mode: &AddressingMode) {
	cpu.register_a = cpu.pop();
	cpu.set_zero_neg_flags(cpu.register_a);
}