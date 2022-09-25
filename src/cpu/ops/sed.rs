// SED - Set Decimal Flag
// D = 1
// Set the decimal mode flag to one.

use super::AddressingMode;
use super::Cpu;

pub const IMOP: u8 = 0xF8;

pub fn sed(cpu: &mut Cpu, _mode: &AddressingMode) {
	cpu.status.set_decimal(true);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_sed() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![IMOP, 0x00]);
		assert!(cpu.status.get_decimal());
	}
}
