// RRA
// Rotate one bit right in memory, then add memory to accumulator (with carry).

use super::AddressingMode;
use super::Cpu;
use super::ror_m_ext;
use super::add_a_carry;

#[allow(dead_code)]
pub const IMOP: u8 = 0x27;

pub fn rra(cpu: &mut Cpu, mode: &AddressingMode) {
	let data = ror_m_ext(cpu, mode);
	add_a_carry(cpu, data);
}