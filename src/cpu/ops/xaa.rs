// XAA - Undefined Behavior

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0x8B;

// Tried replicating the UB
pub fn xaa(cpu: &mut Cpu, mode: &AddressingMode) {
	let addr = cpu.get_operand_address(mode);
	let data = cpu.read(addr);
	cpu.register_a = cpu.register_x;
	cpu.set_zero_neg_flags(cpu.register_a);
	cpu.register_a &= data;
}