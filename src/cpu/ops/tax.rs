use super::CPU;

pub const TAX: u8 = 0xAA;

// TAX - Transfer Accumulator to X
// X = A
// Copies the current contents of the accumulator into the X register and sets the zero and negative flags as appropriate.
impl CPU {
	pub fn tax(&mut self) {
		self.register_x = self.register_a;
		self.set_zero_neg_flags(self.register_x);
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_tax_move_a_to_x() {
		let mut cpu = CPU::new();
		cpu.register_a = 69;
		cpu.interpret(vec![0xaa, 0x00]);
		assert_eq!(cpu.register_x, 69)
	}
}