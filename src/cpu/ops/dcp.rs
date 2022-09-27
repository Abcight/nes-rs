// DCP
// Subtract one from memory.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0xC0;

pub fn dcp(cpu: &mut Cpu, mode: &AddressingMode) {
	let addr = cpu.get_operand_address(mode);
	let mut data = cpu.read(addr);

	data = data.wrapping_sub(1);
	cpu.write(addr, data);

	if data <= cpu.register_a {
		cpu.status.set_carry(true);
	}

	cpu.set_zero_neg_flags(cpu.register_a.wrapping_sub(data));
}
