// DEC - Decrement Memory
// M,Z,N = M-1
// Subtracts one from the value held at a specified memory location setting the zero and negative flags as appropriate.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

pub const IMOP: u8 = 0xC6;

pub fn dec(cpu: &mut Cpu, mode: &AddressingMode) {
	let addr = cpu.get_operand_address(mode);
	let mut data = cpu.read(addr);
	data = data.wrapping_sub(1);
	cpu.write(addr, data);
	cpu.set_zero_neg_flags(data);
}