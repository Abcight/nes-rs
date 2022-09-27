// AXA - AND X register with accumulator
// Perform a logical AND on x register with accumulator, then AND result with 7 and store in memory.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0x9F;

pub fn axa_in(cpu: &mut Cpu, _mode: &AddressingMode) {
	let pos: u8 = cpu.read(cpu.program_counter);
	let addr = cpu.read_u16(pos as u16) + cpu.register_y as u16;
	let data = cpu.register_a & cpu.register_x & 0b0000_0111;
	cpu.write(addr, data);
}

pub fn axa_ab(cpu: &mut Cpu, _mode: &AddressingMode) {
	let addr = cpu.read_u16(cpu.program_counter) + cpu.register_y as u16;
	let data = cpu.register_a & cpu.register_x & 0b0000_0111;
	cpu.write(addr, data);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_axa_indirect() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0b1100_0110, 0xa2, 0b0110_0011, 0x93, 0x00]);
		assert!(cpu.memory.contains(&0b0000_0010));
	}

	#[test]
	fn test_axa_absolute() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0b1100_0110, 0xa2, 0b0110_0011, 0x9f, 0x00]);
		assert!(cpu.memory.contains(&0b0000_0010));
	}
}
