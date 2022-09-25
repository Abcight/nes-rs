// TAY - Transfer Accumulator to Y
// Y = A
// Copies the current contents of the accumulator into the Y register and sets the zero and negative flags as appropriate.

use super::AddressingMode;
use super::Cpu;

#[allow(dead_code)]
pub const IMOP: u8 = 0xA8;

pub fn tay(cpu: &mut Cpu, _mode: &AddressingMode) {
	cpu.register_y = cpu.register_a;
	cpu.set_zero_neg_flags(cpu.register_y);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_tay_move_a_to_y() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xA9, 69, IMOP, 0x00]);
		assert_eq!(cpu.register_y, 69)
	}
}
