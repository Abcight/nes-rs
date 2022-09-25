// TYA - Transfer Y to Accumulator
// A = Y
// Copies the current contents of the Y register into the accumulator and sets the zero and negative flags as appropriate.

use super::AddressingMode;
use super::Cpu;

#[allow(dead_code)]
pub const IMOP: u8 = 0x98;

pub fn tya(cpu: &mut Cpu, _mode: &AddressingMode) {
	cpu.register_a = cpu.register_y;
	cpu.set_zero_neg_flags(cpu.register_a);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_tya_move_y_to_a() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa0, 69, IMOP, 0x00]);
		assert_eq!(cpu.register_a, 69)
	}
}
