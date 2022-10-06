// ASR
// AND byte with accumulator, then shift right one bit in accumu-lator.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0x4B;

impl Cpu {
	pub fn asr(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		let data = self.read(addr);
		self.register_a &= data;
		self.set_zero_neg_flags(self.register_a);
		self.lsr_a(mode);
	}
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
