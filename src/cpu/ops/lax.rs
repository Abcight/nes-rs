use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0xA7;

impl Cpu {
	pub fn lax(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		let data = self.read(addr);
		self.register_x = data;
		self.register_a = self.register_x;
		self.set_zero_neg_flags(self.register_a);
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_lax() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xA7, 0x00]);
		assert_eq!(cpu.register_a, cpu.register_x);
	}
}