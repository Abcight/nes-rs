// Abc: This is unused on NES, but I have implemented it anyway because it's in the spec.
// CLD - Clear Decimal Mode
// D = 0
use super::AddressingMode;
use super::Cpu;

pub const IMOP: u8 = 0xD8;

pub fn cld(cpu: &mut Cpu, _mode: &AddressingMode) {
	cpu.status.set_decimal(false);
}