// STX - Store X Register
// M = X
// Stores the contents of the X register into memory.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0x86;

pub fn stx(cpu: &mut Cpu, mode: &AddressingMode) {
	let addr = cpu.get_operand_address(mode);
	cpu.write(addr, cpu.register_x);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_stx_store_data() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa2, 0x43, IMOP, 0x00]);
		assert!(cpu.memory.contains(&0x43));
	}
}