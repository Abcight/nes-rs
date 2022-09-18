use super::CPU;

pub const LDA: u8 = 0xA9;

// LDA - Load Accumulator
// A,Z,N = M
// Loads a byte of memory into the accumulator setting the zero and negative flags as appropriate.
impl CPU {
	pub fn lda(&mut self, value: u8) {
		self.register_a = value;
		self.set_zero_neg_flags(self.register_a);
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_lda_load_data() {
		let mut cpu = CPU::new();
		cpu.interpret(vec![LDA, 0x43, 0x00]);
		assert_eq!(cpu.register_a, 0x43);
		assert!(cpu.status & 0b0000_0010 == 0b00);
		assert!(cpu.status & 0b1000_0000 == 0);
	}

	#[test]
	fn test_lda_zero_flag() {
		let mut cpu = CPU::new();
		cpu.interpret(vec![LDA, 0x00, 0x00]);
		assert!(cpu.status & 0b0000_0010 == 0b10);
	}
}