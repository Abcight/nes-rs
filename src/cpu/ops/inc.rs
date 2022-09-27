// INC - Increment Memory
// M,Z,N = M+1
// Adds one to the value held at a specified memory location setting the zero and negative flags as appropriate.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0xE6;

pub fn inc_ret(cpu: &mut Cpu, mode: &AddressingMode) -> u8 {
	let addr = cpu.get_operand_address(mode);
	let mut data = cpu.read(addr);
	data = data.wrapping_add(1);
	cpu.write(addr, data);
	cpu.set_zero_neg_flags(data);
	data
}

pub fn inc(cpu: &mut Cpu, mode: &AddressingMode) {
	inc_ret(cpu, mode);
}