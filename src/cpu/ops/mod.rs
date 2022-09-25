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
pub mod tya;
pub mod sta;
pub mod adc;
pub mod bcc;
pub mod bcs;
pub mod beq;
pub mod bmi;
pub mod bne;
pub mod bpl;
pub mod bvc;
pub mod bvs;
pub mod clc;
pub mod bit;
pub mod cld;
pub mod cli;
pub mod clv;
pub mod cmp;
pub mod dey;
pub mod iny;
pub mod jmp;
pub mod dec;
pub mod inc;
pub mod rts;
pub mod sbc;
pub mod cpx;
pub mod cpy;
pub mod eor;
pub mod sec;
pub mod sed;
pub mod sei;
pub mod stx;
pub mod sty;
pub mod tsx;
pub mod txs;
pub mod jsr;
pub mod lsr;
pub mod ora;
pub mod pha;
pub mod pla;
pub mod plp;
pub mod rol;
pub mod ror;
pub mod rti;

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
		map.insert(0xEA, OpCodeDef::new(1, 2, AddressingMode::Implied, 		|_,_|{}));
		map.insert(0xAA, OpCodeDef::new(1, 2, AddressingMode::Implied, 		tax::tax));
		map.insert(0xA8, OpCodeDef::new(1, 2, AddressingMode::Implied, 		tay::tay));
		map.insert(0xE8, OpCodeDef::new(1, 2, AddressingMode::Implied, 		inx::inx));
		map.insert(0xC8, OpCodeDef::new(1, 2, AddressingMode::Implied, 		iny::iny));
		map.insert(0xCA, OpCodeDef::new(1, 2, AddressingMode::Implied, 		dex::dex));
		map.insert(0x88, OpCodeDef::new(1, 2, AddressingMode::Implied, 		dey::dey));
		map.insert(0x8A, OpCodeDef::new(1, 2, AddressingMode::Implied, 		txa::txa));
		map.insert(0x98, OpCodeDef::new(1, 2, AddressingMode::Implied, 		tya::tya));
		map.insert(0x90, OpCodeDef::new(2, 2, AddressingMode::Implied, 		bcc::bcc));
		map.insert(0xB0, OpCodeDef::new(2, 2, AddressingMode::Implied, 		bcs::bcs));
		map.insert(0xF0, OpCodeDef::new(2, 2, AddressingMode::Implied, 		beq::beq));
		map.insert(0x30, OpCodeDef::new(2, 2, AddressingMode::Implied, 		bmi::bmi));
		map.insert(0xD0, OpCodeDef::new(2, 2, AddressingMode::Implied, 		bne::bne));
		map.insert(0x10, OpCodeDef::new(2, 2, AddressingMode::Implied, 		bpl::bpl));
		map.insert(0x50, OpCodeDef::new(2, 2, AddressingMode::Implied, 		bvc::bvc));
		map.insert(0x70, OpCodeDef::new(2, 2, AddressingMode::Implied, 		bvs::bvs));
		map.insert(0x18, OpCodeDef::new(1, 2, AddressingMode::Implied, 		clc::clc));
		map.insert(0xD8, OpCodeDef::new(1, 2, AddressingMode::Implied, 		cld::cld));
		map.insert(0x58, OpCodeDef::new(1, 2, AddressingMode::Implied, 		cli::cli));
		map.insert(0xB8, OpCodeDef::new(1, 2, AddressingMode::Implied, 		clv::clv));
		map.insert(0x60, OpCodeDef::new(1, 6, AddressingMode::Implied, 		rts::rts));
		map.insert(0x38, OpCodeDef::new(1, 2, AddressingMode::Implied, 		sec::sec));
		map.insert(0xF8, OpCodeDef::new(1, 2, AddressingMode::Implied, 		sed::sed));
		map.insert(0x78, OpCodeDef::new(1, 2, AddressingMode::Implied, 		sei::sei));
		map.insert(0xBA, OpCodeDef::new(1, 2, AddressingMode::Implied, 		tsx::tsx));
		map.insert(0x9A, OpCodeDef::new(1, 2, AddressingMode::Implied, 		txs::txs));
		map.insert(0x20, OpCodeDef::new(3, 6, AddressingMode::Absolute, 	jsr::jsr));
		map.insert(0x48, OpCodeDef::new(1, 3, AddressingMode::Implied, 		pha::pha));
		map.insert(0x68, OpCodeDef::new(1, 4, AddressingMode::Implied, 		pla::pla));
		map.insert(0x28, OpCodeDef::new(1, 4, AddressingMode::IndirectY, 	plp::plp));
		map.insert(0x40, OpCodeDef::new(1, 6, AddressingMode::Implied,	 	rti::rti));

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

		map.insert(0x49, OpCodeDef::new(2, 2, AddressingMode::Immediate, 	eor::eor));
		map.insert(0x45, OpCodeDef::new(2, 3, AddressingMode::ZeroPage, 	eor::eor));
		map.insert(0x55, OpCodeDef::new(2, 4, AddressingMode::ZeroPageX, 	eor::eor));
		map.insert(0x4D, OpCodeDef::new(3, 4, AddressingMode::Absolute, 	eor::eor));
		map.insert(0x5D, OpCodeDef::new(3, 4, AddressingMode::AbsoluteX, 	eor::eor));
		map.insert(0x59, OpCodeDef::new(3, 4, AddressingMode::AbsoluteY, 	eor::eor));
		map.insert(0x41, OpCodeDef::new(2, 6, AddressingMode::IndirectX, 	eor::eor));
		map.insert(0x51, OpCodeDef::new(2, 5, AddressingMode::IndirectY, 	eor::eor));
		
		map.insert(0x85, OpCodeDef::new(2, 3, AddressingMode::ZeroPage, 	sta::sta));
		map.insert(0x95, OpCodeDef::new(2, 4, AddressingMode::ZeroPageX, 	sta::sta));
		map.insert(0x8D, OpCodeDef::new(3, 4, AddressingMode::Absolute, 	sta::sta));
		map.insert(0x9D, OpCodeDef::new(3, 5, AddressingMode::AbsoluteX, 	sta::sta));
		map.insert(0x99, OpCodeDef::new(3, 5, AddressingMode::AbsoluteY, 	sta::sta));
		map.insert(0x81, OpCodeDef::new(2, 6, AddressingMode::IndirectX, 	sta::sta));
		map.insert(0x91, OpCodeDef::new(2, 6, AddressingMode::IndirectY, 	sta::sta));

		map.insert(0x69, OpCodeDef::new(2, 2, AddressingMode::Immediate, 	adc::adc));
		map.insert(0x65, OpCodeDef::new(2, 3, AddressingMode::ZeroPage, 	adc::adc));
		map.insert(0x75, OpCodeDef::new(2, 4, AddressingMode::ZeroPageX, 	adc::adc));
		map.insert(0x6D, OpCodeDef::new(3, 4, AddressingMode::Absolute, 	adc::adc));
		map.insert(0x7D, OpCodeDef::new(3, 4, AddressingMode::AbsoluteX, 	adc::adc));
		map.insert(0x79, OpCodeDef::new(3, 4, AddressingMode::AbsoluteY, 	adc::adc));
		map.insert(0x61, OpCodeDef::new(2, 6, AddressingMode::IndirectX, 	adc::adc));
		map.insert(0x71, OpCodeDef::new(2, 5, AddressingMode::IndirectY, 	adc::adc));

		map.insert(0xE9, OpCodeDef::new(2, 2, AddressingMode::Immediate, 	sbc::sbc));
		map.insert(0xE5, OpCodeDef::new(2, 3, AddressingMode::ZeroPage, 	sbc::sbc));
		map.insert(0xF5, OpCodeDef::new(2, 4, AddressingMode::ZeroPageX, 	sbc::sbc));
		map.insert(0xED, OpCodeDef::new(3, 4, AddressingMode::Absolute, 	sbc::sbc));
		map.insert(0xFD, OpCodeDef::new(3, 4, AddressingMode::AbsoluteX, 	sbc::sbc));
		map.insert(0xF9, OpCodeDef::new(3, 4, AddressingMode::AbsoluteY, 	sbc::sbc));
		map.insert(0xE1, OpCodeDef::new(2, 6, AddressingMode::IndirectX, 	sbc::sbc));
		map.insert(0xF1, OpCodeDef::new(2, 5, AddressingMode::IndirectY, 	sbc::sbc));

		map.insert(0x24, OpCodeDef::new(2, 3, AddressingMode::ZeroPage, 	bit::bit));
		map.insert(0x2C, OpCodeDef::new(3, 4, AddressingMode::Absolute, 	bit::bit));

		map.insert(0xC9, OpCodeDef::new(2, 2, AddressingMode::Immediate, 	cmp::cmp));
		map.insert(0xC5, OpCodeDef::new(2, 3, AddressingMode::ZeroPage, 	cmp::cmp));
		map.insert(0xD5, OpCodeDef::new(2, 4, AddressingMode::ZeroPageX, 	cmp::cmp));
		map.insert(0xCD, OpCodeDef::new(3, 4, AddressingMode::Absolute, 	cmp::cmp));
		map.insert(0xDD, OpCodeDef::new(3, 4, AddressingMode::AbsoluteX, 	cmp::cmp));
		map.insert(0xD9, OpCodeDef::new(3, 4, AddressingMode::AbsoluteY, 	cmp::cmp));
		map.insert(0xC1, OpCodeDef::new(2, 6, AddressingMode::IndirectX, 	cmp::cmp));
		map.insert(0xD1, OpCodeDef::new(2, 5, AddressingMode::IndirectY, 	cmp::cmp));

		map.insert(0xE0, OpCodeDef::new(2, 2, AddressingMode::Immediate, 	cpx::cpx));
		map.insert(0xE4, OpCodeDef::new(2, 3, AddressingMode::ZeroPage, 	cpx::cpx));
		map.insert(0xEC, OpCodeDef::new(3, 4, AddressingMode::Absolute, 	cpx::cpx));

		map.insert(0xC0, OpCodeDef::new(2, 2, AddressingMode::Immediate, 	cpy::cpy));
		map.insert(0xC4, OpCodeDef::new(2, 3, AddressingMode::ZeroPage, 	cpy::cpy));
		map.insert(0xCC, OpCodeDef::new(3, 4, AddressingMode::Absolute, 	cpy::cpy));

		map.insert(0x4C, OpCodeDef::new(3, 3, AddressingMode::Absolute, 	jmp::jmp));
		map.insert(0x6C, OpCodeDef::new(3, 5, AddressingMode::Immediate, 	jmp::jmp));

		map.insert(0xC6, OpCodeDef::new(2, 5, AddressingMode::ZeroPage, 	dec::dec));
		map.insert(0xD6, OpCodeDef::new(2, 6, AddressingMode::ZeroPageX, 	dec::dec));
		map.insert(0xCE, OpCodeDef::new(3, 6, AddressingMode::Absolute, 	dec::dec));
		map.insert(0xDE, OpCodeDef::new(3, 7, AddressingMode::AbsoluteX, 	dec::dec));

		map.insert(0xE6, OpCodeDef::new(2, 5, AddressingMode::ZeroPage, 	inc::inc));
		map.insert(0xF6, OpCodeDef::new(2, 6, AddressingMode::ZeroPageX, 	inc::inc));
		map.insert(0xEE, OpCodeDef::new(3, 6, AddressingMode::Absolute, 	inc::inc));
		map.insert(0xFE, OpCodeDef::new(3, 7, AddressingMode::AbsoluteX, 	inc::inc));

		map.insert(0x86, OpCodeDef::new(2, 3, AddressingMode::ZeroPage, 	stx::stx));
		map.insert(0x96, OpCodeDef::new(2, 4, AddressingMode::ZeroPageY, 	stx::stx));
		map.insert(0x8E, OpCodeDef::new(3, 4, AddressingMode::Absolute, 	stx::stx));

		map.insert(0x84, OpCodeDef::new(2, 3, AddressingMode::ZeroPage, 	sty::sty));
		map.insert(0x94, OpCodeDef::new(2, 4, AddressingMode::ZeroPageX, 	sty::sty));
		map.insert(0x8C, OpCodeDef::new(3, 4, AddressingMode::Absolute, 	sty::sty));

		map.insert(0x4A, OpCodeDef::new(1, 2, AddressingMode::Implied, 		lsr::lsr_a));
		map.insert(0x46, OpCodeDef::new(2, 5, AddressingMode::ZeroPage, 	lsr::lsr_m));
		map.insert(0x56, OpCodeDef::new(2, 6, AddressingMode::ZeroPageX, 	lsr::lsr_m));
		map.insert(0x4E, OpCodeDef::new(3, 6, AddressingMode::Absolute, 	lsr::lsr_m));
		map.insert(0x5E, OpCodeDef::new(3, 7, AddressingMode::AbsoluteX, 	lsr::lsr_m));

		map.insert(0x09, OpCodeDef::new(2, 2, AddressingMode::Immediate, 	ora::ora));
		map.insert(0x05, OpCodeDef::new(2, 3, AddressingMode::ZeroPage, 	ora::ora));
		map.insert(0x15, OpCodeDef::new(2, 4, AddressingMode::ZeroPageX, 	ora::ora));
		map.insert(0x0D, OpCodeDef::new(3, 4, AddressingMode::Absolute, 	ora::ora));
		map.insert(0x1D, OpCodeDef::new(3, 4, AddressingMode::AbsoluteX, 	ora::ora));
		map.insert(0x19, OpCodeDef::new(3, 4, AddressingMode::AbsoluteY, 	ora::ora));
		map.insert(0x01, OpCodeDef::new(2, 6, AddressingMode::IndirectX, 	ora::ora));
		map.insert(0x11, OpCodeDef::new(2, 5, AddressingMode::IndirectY, 	ora::ora));

		map.insert(0x2A, OpCodeDef::new(1, 2, AddressingMode::Implied, 		rol::rol_a));
		map.insert(0x26, OpCodeDef::new(2, 5, AddressingMode::ZeroPage, 	rol::rol_m));
		map.insert(0x36, OpCodeDef::new(2, 6, AddressingMode::ZeroPageX, 	rol::rol_m));
		map.insert(0x2E, OpCodeDef::new(3, 6, AddressingMode::Absolute, 	rol::rol_m));
		map.insert(0x3E, OpCodeDef::new(3, 7, AddressingMode::AbsoluteX, 	rol::rol_m));

		map.insert(0x6A, OpCodeDef::new(1, 2, AddressingMode::Implied,		ror::ror_a));
		map.insert(0x66, OpCodeDef::new(2, 5, AddressingMode::ZeroPage,		ror::ror_m));
		map.insert(0x76, OpCodeDef::new(2, 6, AddressingMode::ZeroPageX,	ror::ror_m));
		map.insert(0x6E, OpCodeDef::new(3, 6, AddressingMode::Absolute,		ror::ror_m));
		map.insert(0x7E, OpCodeDef::new(3, 7, AddressingMode::AbsoluteX,	ror::ror_m));

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
