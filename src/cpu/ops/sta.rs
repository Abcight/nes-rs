// STA - Store Accumulator
// M = A
// Stores the contents of the accumulator into memory.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0x85;

pub fn sta(cpu: &mut Cpu, mode: &AddressingMode) {
	let addr = cpu.get_operand_address(mode);
	cpu.write(addr, cpu.register_a);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_sta_store_data() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0x43, IMOP, 0x00]);
		assert!(cpu.memory.contains(&0x43));
	}
}
