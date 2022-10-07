// AXS - AND X register with accumulator
// Perform a logical AND on X register with accumulator and store result in X register, then subtract byte from X register (without borrow).

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0xCB;

impl Cpu {
	pub fn axs(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		let data = self.read(addr);
		let xa = self.register_x & self.register_a;
		let result = xa.wrapping_sub(data);

		if data <= xa {
			self.status.set_carry(true);
		}

		self.register_x = result;
		self.set_zero_neg_flags(self.register_x);
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_axs() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0b0000_1111, 0xa2, 0b0000_0111, IMOP, 0b0000_0001, 0x00]);
		assert_eq!(cpu.register_x, 0b0000_0110)
	}
}
