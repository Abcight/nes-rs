use std::collections::HashMap;

use lazy_static::lazy_static;

use super::memory::*;
use super::AddressingMode;
use super::Cpu;

pub mod dex;
pub mod inx;
pub mod ldx;
pub mod lda;
pub mod ldy;
pub mod tax;
pub mod tay;
pub mod asl;
pub mod and;
pub mod txa;

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
		map.insert(0xA8, OpCodeDef::new(1, 2, AddressingMode::Implied, 		tay::tay));
		map.insert(0xE8, OpCodeDef::new(1, 2, AddressingMode::Implied, 		inx::inx));
		map.insert(0xCA, OpCodeDef::new(1, 2, AddressingMode::Implied, 		dex::dex));
		map.insert(0x8A, OpCodeDef::new(1, 2, AddressingMode::Implied, 		txa::txa));

		map.insert(0xA9, OpCodeDef::new(2, 2, AddressingMode::Immediate, 	lda::lda));
		map.insert(0xA5, OpCodeDef::new(2, 3, AddressingMode::ZeroPage, 	lda::lda));
		map.insert(0xB5, OpCodeDef::new(2, 4, AddressingMode::ZeroPageX, 	lda::lda));
		map.insert(0xAD, OpCodeDef::new(3, 4, AddressingMode::Absolute, 	lda::lda));
		map.insert(0xBD, OpCodeDef::new(3, 4, AddressingMode::AbsoluteX, 	lda::lda));
		map.insert(0xB9, OpCodeDef::new(3, 4, AddressingMode::AbsoluteY, 	lda::lda));
		map.insert(0xA1, OpCodeDef::new(2, 6, AddressingMode::IndirectX, 	lda::lda));
		map.insert(0xB1, OpCodeDef::new(2, 5, AddressingMode::IndirectY, 	lda::lda));

		map.insert(0xA2, OpCodeDef::new(2, 2, AddressingMode::Immediate, 	ldx::ldx));
		map.insert(0xA6, OpCodeDef::new(2, 3, AddressingMode::ZeroPage, 	ldx::ldx));
		map.insert(0xB6, OpCodeDef::new(2, 4, AddressingMode::ZeroPageY, 	ldx::ldx));
		map.insert(0xAE, OpCodeDef::new(3, 4, AddressingMode::Absolute, 	ldx::ldx));
		map.insert(0xBE, OpCodeDef::new(3, 4, AddressingMode::AbsoluteY, 	ldx::ldx));

		map.insert(0xA0, OpCodeDef::new(2, 2, AddressingMode::Immediate, 	ldy::ldy));
		map.insert(0xA4, OpCodeDef::new(2, 3, AddressingMode::ZeroPage, 	ldy::ldy));
		map.insert(0xB4, OpCodeDef::new(2, 4, AddressingMode::ZeroPageX, 	ldy::ldy));
		map.insert(0xAC, OpCodeDef::new(3, 4, AddressingMode::Absolute, 	ldy::ldy));
		map.insert(0xBC, OpCodeDef::new(3, 4, AddressingMode::AbsoluteX, 	ldy::ldy));

		map.insert(0x0A, OpCodeDef::new(1, 2, AddressingMode::Immediate, 	asl::asl));
		map.insert(0x06, OpCodeDef::new(2, 5, AddressingMode::ZeroPage, 	asl::asl));
		map.insert(0x16, OpCodeDef::new(2, 6, AddressingMode::ZeroPageX, 	asl::asl));
		map.insert(0x0E, OpCodeDef::new(3, 6, AddressingMode::Absolute, 	asl::asl));
		map.insert(0x1E, OpCodeDef::new(3, 7, AddressingMode::AbsoluteX, 	asl::asl));
		
		map.insert(0x29, OpCodeDef::new(2, 2, AddressingMode::Immediate, 	and::and));
		map.insert(0x25, OpCodeDef::new(2, 3, AddressingMode::ZeroPage, 	and::and));
		map.insert(0x35, OpCodeDef::new(2, 4, AddressingMode::ZeroPageX, 	and::and));
		map.insert(0x2D, OpCodeDef::new(3, 4, AddressingMode::Absolute, 	and::and));
		map.insert(0x3D, OpCodeDef::new(3, 4, AddressingMode::AbsoluteX, 	and::and));
		map.insert(0x39, OpCodeDef::new(3, 4, AddressingMode::AbsoluteY, 	and::and));
		map.insert(0x21, OpCodeDef::new(2, 6, AddressingMode::IndirectX, 	and::and));
		map.insert(0x31, OpCodeDef::new(2, 5, AddressingMode::IndirectY, 	and::and));
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
