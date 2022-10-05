// SRE
// Shift right one bit in memory, then EOR accumulator with memory.

use super::AddressingMode;
use super::Cpu;
use super::lsr_m_ext;

#[allow(dead_code)]
pub const IMOP: u8 = 0x47;

pub fn sre(cpu: &mut Cpu, mode: &AddressingMode) {
	let data = lsr_m_ext(cpu, mode);
	cpu.register_a ^= data;
	cpu.set_zero_neg_flags(cpu.register_a);
}