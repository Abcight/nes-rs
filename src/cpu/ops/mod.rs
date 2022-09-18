use std::collections::HashMap;

use lazy_static::lazy_static;

use super::AddressingMode;
use super::Cpu;

pub mod dex;
pub mod inx;
pub mod lda;
pub mod tax;

pub struct OpCodeDef {
	pub len: u8,
	pub cycles: u8,
	pub mode: AddressingMode,
	pub instruction: fn(&mut Cpu, &AddressingMode),
}

impl OpCodeDef {
	pub fn new(
		len: u8,
		cycles: u8,
		mode: AddressingMode,
		instruction: fn(&mut Cpu, &AddressingMode),
	) -> Self {
		Self {
			len,
			cycles,
			mode,
			instruction,
		}
	}

	pub fn execute(&self, cpu: &mut Cpu) {
		let instruction = self.instruction;
		instruction(cpu, &self.mode);
	}
}

lazy_static! {
	#[rustfmt::skip]
	static ref INSTRUCTIONS: HashMap<u8, OpCodeDef> = {
		let mut map = HashMap::new();
		map.insert(0xAA, OpCodeDef::new(1, 2, AddressingMode::Implied, 		tax::tax));
		map.insert(0xE8, OpCodeDef::new(1, 2, AddressingMode::Implied, 		inx::inx));
		map.insert(0xCA, OpCodeDef::new(1, 2, AddressingMode::Implied, 		dex::dex));

		map.insert(0xA9, OpCodeDef::new(2, 2, AddressingMode::Immediate, 	lda::lda));
		map.insert(0xA5, OpCodeDef::new(2, 3, AddressingMode::ZeroPage, 	lda::lda));
		map.insert(0xB5, OpCodeDef::new(2, 4, AddressingMode::ZeroPageX, 	lda::lda));
		map.insert(0xAD, OpCodeDef::new(3, 4, AddressingMode::Absolute, 	lda::lda));
		map.insert(0xBD, OpCodeDef::new(3, 4, AddressingMode::AbsoluteX, 	lda::lda));
		map.insert(0xB9, OpCodeDef::new(3, 4, AddressingMode::AbsoluteY, 	lda::lda));
		map.insert(0xA1, OpCodeDef::new(2, 6, AddressingMode::IndirectX, 	lda::lda));
		map.insert(0xB1, OpCodeDef::new(2, 5, AddressingMode::IndirectY, 	lda::lda));
		map
	};
}

#[rustfmt::skip]
pub fn get_instruction_def<'a>(code: u8) -> &'a OpCodeDef {
	if let Some(def) = INSTRUCTIONS.get(&code) {
		def
	} else {
		panic!("Unsupported code {}", code);
	}
}
