use super::CPU;

pub const INX: u8 = 0xE8;

// INX - Increment X Register
// X,Z,N = X + 1
// Adds one to the X register setting the zero and negative flags as appropriate.
impl CPU {
	pub fn inx(&mut self) {
		self.register_x = self.register_x.overflowing_add(1).0;
		self.set_zero_neg_flags(self.register_x);
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_inx_increment() {
		let mut cpu = CPU::new();
		cpu.register_x = 10;
		cpu.interpret(vec![INX, 0x00]);
		assert_eq!(cpu.register_x, 11)
	}

	#[test]
	fn test_inx_overflow() {
		let mut cpu = CPU::new();
		cpu.register_x = 0xff;
		cpu.interpret(vec![INX, INX, 0x00]);
		assert_eq!(cpu.register_x, 1)
	}
}