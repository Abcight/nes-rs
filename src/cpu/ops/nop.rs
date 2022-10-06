// NOP - No operation
// Doesn't perform any operation

use super::AddressingMode;
use super::Cpu;

#[allow(dead_code)]
pub const IMOP: u8 = 0xEA;

impl Cpu {
	pub fn nop(&mut self, _mode: &AddressingMode) {}
}