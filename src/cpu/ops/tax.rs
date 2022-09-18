// TAX - Transfer Accumulator to X
// X = A
// Copies the current contents of the accumulator into the X register and sets the zero and negative flags as appropriate.

use super::AddressingMode;
use super::Cpu;

pub const IMOP: u8 = 0xAA;

pub fn tax(cpu: &mut Cpu, _mode: &AddressingMode) {
	cpu.register_x = cpu.register_a;
	cpu.set_zero_neg_flags(cpu.register_x);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_tax_move_a_to_x() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 69, IMOP, 0x00]);
		assert_eq!(cpu.register_x, 69)
	}
}
