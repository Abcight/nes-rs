// TXA - Transfer X to Accumulator
// A = X
// Copies the current contents of the X register into the accumulator and sets the zero and negative flags as appropriate.

use super::AddressingMode;
use super::Cpu;

pub const IMOP: u8 = 0x8A;

pub fn txa(cpu: &mut Cpu, _mode: &AddressingMode) {
	println!("{}", cpu.register_x);
	cpu.register_a = cpu.register_x;
	println!("{}", cpu.register_a);
	cpu.set_zero_neg_flags(cpu.register_a);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_txa_move_x_to_a() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa2, 69, IMOP, 0x00]);
		assert_eq!(cpu.register_a, 69)
	}
}
