// ASR
// AND byte with accumulator, then shift right one bit in accumu-lator.

use super::AddressingMode;
use super::Cpu;
use super::Memory;
use super::lsr;

#[allow(dead_code)]
pub const IMOP: u8 = 0x4B;

pub fn asr(cpu: &mut Cpu, mode: &AddressingMode) {
	let addr = cpu.get_operand_address(mode);
	let data = cpu.read(addr);
	cpu.register_a &= data;
	cpu.set_zero_neg_flags(cpu.register_a);
	lsr::lsr_a(cpu, mode);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_asr() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0b1000_0011, IMOP, 0b1000_0001, 0x00]);
		assert_eq!(cpu.register_a, 0b0100_0000)
	}
}
