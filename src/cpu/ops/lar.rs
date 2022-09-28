// LAR
// AND memory with stack pointer, transfer result to accumulator, X register and stack pointer.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0xBB;

pub fn lar(cpu: &mut Cpu, mode: &AddressingMode) {
	let addr = cpu.get_operand_address(mode);
	let data = cpu.read(addr) & cpu.stack_pointer;
	cpu.register_a = data;
	cpu.register_x = data;
	cpu.stack_pointer = data;
	cpu.set_zero_neg_flags(data);
}