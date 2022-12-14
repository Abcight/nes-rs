// BIT - Bit Test
// A & M, N = M7, V = M6
// This instruction is used to test if one or more bits are set in a target memory location. The mask pattern in A is ANDed with the value in memory to set or clear the zero flag, but the result is not kept. Bits 7 and 6 of the value from memory are copied into the N and V flags.

use super::AddressingMode;
use super::Cpu;
use super::Memory;

#[allow(dead_code)]
pub const IMOP: u8 = 0x24;

impl Cpu {
	pub fn bit(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		let data = self.read(addr);
		println!("{data}");
		self.status.set_zero(self.register_a & data == 0);
		self.status.set_negative(data & 0b1000_0000 > 0);
		self.status.set_overflow(data & 0b0100_0000 > 0);
	}
}
