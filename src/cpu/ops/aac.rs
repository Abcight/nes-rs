// AAC - AND byte with accumulator
// If result is negative then carry is set.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0x0B;

pub fn aac(cpu: &mut Cpu, mode: &AddressingMode) {
	let addr = cpu.get_operand_address(mode);
	let data = cpu.read(addr);
	cpu.register_a &= data;
	cpu.set_zero_neg_flags(cpu.register_a);
	cpu.status.set_carry(cpu.status.get_negative());
}


#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_aac() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0b1000_0001, IMOP, 0b1000_0100, 0x00]);
		assert_eq!(cpu.register_a, 0b1000_0000);
		assert!(cpu.status.get_negative())
	}
}