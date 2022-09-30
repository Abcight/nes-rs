// RLA
// Rotate one bit left in memory, then AND accumulator with memory.

use super::AddressingMode;
use super::Cpu;
use super::rol_m_ext;

#[allow(dead_code)]
pub const IMOP: u8 = 0x27;

pub fn rla(cpu: &mut Cpu, mode: &AddressingMode) {
	let data = rol_m_ext(cpu, mode);
	cpu.register_a &= data;
	cpu.set_zero_neg_flags(cpu.register_a);
}