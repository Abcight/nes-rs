// ISC - INC subtract
// Increase memory by one, then subtract memory from accu-mulator (with borrow)

use super::AddressingMode;
use super::Cpu;
use super::inc_ret;

#[allow(dead_code)]
pub const IMOP: u8 = 0xC8;

pub fn isc(cpu: &mut Cpu, mode: &AddressingMode) {
	let data = (inc_ret(cpu, mode) as i8).wrapping_neg().wrapping_sub(1) as u8;
	let sum = cpu.register_a as u16 + data as u16 + cpu.status.get_carry() as u16;
	let carry = sum > 0xff;

	cpu.status.set_carry(carry);
	let result = sum as u8;

	cpu.status.set_overflow((data ^ result) & (result ^ cpu.register_a) & 0x80 != 0);

	cpu.register_a = result;
	cpu.set_zero_neg_flags(cpu.register_a);
}