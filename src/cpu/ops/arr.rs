// ARR - AND byte with accumulator
// AND byte with accumulator, then rotate one bit right in accumulator and check bit 5 and 6.

use super::AddressingMode;
use super::Cpu;
use super::Memory;
use super::ror;

#[allow(dead_code)]
pub const IMOP: u8 = 0x6B;

pub fn arr(cpu: &mut Cpu, mode: &AddressingMode) {
	let addr = cpu.get_operand_address(mode);
	let data = cpu.read(addr);

	cpu.register_a &= data;
	cpu.set_zero_neg_flags(cpu.register_a);

	ror::ror_a(cpu, mode);

	let result = cpu.register_a;
	let b5 = (result >> 5) & 1;
	let b6 = (result >> 6) & 1;
	cpu.status.set_carry(b5 == 1);
	cpu.status.set_overflow(b5 ^ b6 == 1);
	cpu.set_zero_neg_flags(result);
}