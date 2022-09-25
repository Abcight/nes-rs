// STY - Store Y Register
// M = Y
// Stores the contents of the Y register into memory.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

pub const IMOP: u8 = 0x84;

pub fn sty(cpu: &mut Cpu, mode: &AddressingMode) {
	let addr = cpu.get_operand_address(mode);
	cpu.write(addr, cpu.register_y);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_sty_store_data() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa0, 0x43, IMOP, 0x00]);
		assert!(cpu.memory.contains(&0x43));
	}
}