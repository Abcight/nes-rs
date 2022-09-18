use super::CPU;

pub const DEX: u8 = 0xCA;

// INX - Increment X Register
// X,Z,N = X + 1
// Adds one to the X register setting the zero and negative flags as appropriate.
impl CPU {
	pub fn dex(&mut self) {
		self.register_x = self.register_x.overflowing_sub(1).0;
		self.set_zero_neg_flags(self.register_x);
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_dex_decrement() {
		let mut cpu = CPU::new();
		cpu.register_x = 10;
		cpu.interpret(vec![DEX, 0x00]);
		assert_eq!(cpu.register_x, 9)
	}

	#[test]
	fn test_inx_overflow() {
		let mut cpu = CPU::new();
		cpu.register_x = 0x00;
		cpu.interpret(vec![DEX, DEX, 0x00]);
		assert_eq!(cpu.register_x, 0xff)
	}
}