use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0xA7;

pub fn lax(cpu: &mut Cpu, mode: &AddressingMode) {
	let addr = cpu.get_operand_address(mode);
	let data = cpu.read(addr);
	cpu.register_x = data;
	cpu.register_a = cpu.register_x;
	cpu.set_zero_neg_flags(cpu.register_a);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_lax() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xA7, 0x00]);
		assert_eq!(cpu.register_a, cpu.register_x);
	}
}