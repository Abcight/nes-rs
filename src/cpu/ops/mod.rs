use super::memory::*;
use super::AddressingMode;
use super::Cpu;

#[macro_export]
macro_rules! incl {
	( $( $x:ident ),* ) => { $(pub mod $x;)* };
}

incl!(
	//						 nop					  brk
	adc, and, asl, bcc, bcs, beq, bit, bmi, bne, bpl,	   bvc, bvs, clc,
	cld, cli, clv, cmp, cpx, cpy, dec, dex, dey, eor, inc, inx, iny, jmp,
	jsr, lda, ldx, ldy, lsr, 	  ora, pha, php, pla, plp, rol, ror, rti,
	rts, sbc, sec, sed, sei, sta, stx, sty, tax, tay, tsx, txa, txs, tya
);

pub struct OpCodeDef {
	pub len: u8,
	pub cycles: u8,
	pub mode: AddressingMode,
	pub instruction: fn(&mut Cpu, &AddressingMode),
}

impl OpCodeDef {
	pub fn execute(&self, cpu: &mut Cpu) {
		let instruction = self.instruction;
		instruction(cpu, &self.mode);
	}
}

#[macro_export]
macro_rules! map {
	( $( $ident:ident, $op:expr, $len:expr, $cycles:expr, $mode:expr, $fn:expr ),* ) => {
		mod ops_gen {
			use super::*;
			pub const UNSUPPORTED: OpCodeDef = OpCodeDef {
				len: 1,
				cycles: 2,
				mode: AddressingMode::Implied,
				instruction: (|_, _| {}),
			};
			$(
				pub const $ident: OpCodeDef = OpCodeDef { len: $len, cycles: $cycles, mode: $mode, instruction: $fn };
			)*
		}

		pub const fn get_instruction_def<'a>(code: u8) -> &'a OpCodeDef {
			match code { 
				$($op => &ops_gen::$ident,)*
				_ => &ops_gen::UNSUPPORTED
			}
		}
	};
}

map!(
	NOP, 0xEA, 1, 2, AddressingMode::Implied,		|_,_|{},
	TAX, 0xAA, 1, 2, AddressingMode::Implied,		tax::tax,
	TAY, 0xA8, 1, 2, AddressingMode::Implied,		tay::tay,
	INX, 0xE8, 1, 2, AddressingMode::Implied,		inx::inx,
	INY, 0xC8, 1, 2, AddressingMode::Implied,		iny::iny,
	DEX, 0xCA, 1, 2, AddressingMode::Implied,		dex::dex,
	DEY, 0x88, 1, 2, AddressingMode::Implied,		dey::dey,
	TXA, 0x8A, 1, 2, AddressingMode::Implied,		txa::txa,
	TYA, 0x98, 1, 2, AddressingMode::Implied,		tya::tya,
	BCC, 0x90, 2, 2, AddressingMode::Implied,		bcc::bcc,
	BCS, 0xB0, 2, 2, AddressingMode::Implied,		bcs::bcs,
	BEQ, 0xF0, 2, 2, AddressingMode::Implied,		beq::beq,
	BMI, 0x30, 2, 2, AddressingMode::Implied,		bmi::bmi,
	BNE, 0xD0, 2, 2, AddressingMode::Implied,		bne::bne,
	BPL, 0x10, 2, 2, AddressingMode::Implied,		bpl::bpl,
	BVC, 0x50, 2, 2, AddressingMode::Implied,		bvc::bvc,
	BVS, 0x70, 2, 2, AddressingMode::Implied,		bvs::bvs,
	CLC, 0x18, 1, 2, AddressingMode::Implied,		clc::clc,
	CLD, 0xD8, 1, 2, AddressingMode::Implied,		cld::cld,
	CLI, 0x58, 1, 2, AddressingMode::Implied,		cli::cli,
	CLV, 0xB8, 1, 2, AddressingMode::Implied,		clv::clv,
	RTS, 0x60, 1, 6, AddressingMode::Implied,		rts::rts,
	SEC, 0x38, 1, 2, AddressingMode::Implied,		sec::sec,
	SED, 0xF8, 1, 2, AddressingMode::Implied,		sed::sed,
	SEI, 0x78, 1, 2, AddressingMode::Implied,		sei::sei,
	TSX, 0xBA, 1, 2, AddressingMode::Implied,		tsx::tsx,
	TXS, 0x9A, 1, 2, AddressingMode::Implied,		txs::txs,
	JSR, 0x20, 3, 6, AddressingMode::Absolute,		jsr::jsr,
	PHA, 0x48, 1, 3, AddressingMode::Implied,		pha::pha,
	PLA, 0x68, 1, 4, AddressingMode::Implied,		pla::pla,
	PLP, 0x28, 1, 4, AddressingMode::IndirectY,		plp::plp,
	RTI, 0x40, 1, 6, AddressingMode::Implied,		rti::rti,
	PHP, 0x08, 1, 3, AddressingMode::Implied,		php::php,

	LDA1, 0xA9, 2, 2, AddressingMode::Immediate,	lda::lda,
	LDA2, 0xA5, 2, 3, AddressingMode::ZeroPage,		lda::lda,
	LDA3, 0xB5, 2, 4, AddressingMode::ZeroPageX,	lda::lda,
	LDA4, 0xAD, 3, 4, AddressingMode::Absolute,		lda::lda,
	LDA5, 0xBD, 3, 4, AddressingMode::AbsoluteX,	lda::lda,
	LDA6, 0xB9, 3, 4, AddressingMode::AbsoluteY,	lda::lda,
	LDA7, 0xA1, 2, 6, AddressingMode::IndirectX,	lda::lda,
	LDA8, 0xB1, 2, 5, AddressingMode::IndirectY,	lda::lda,

	LDX1, 0xA2, 2, 2, AddressingMode::Immediate,	ldx::ldx,
	LDX2, 0xA6, 2, 3, AddressingMode::ZeroPage,		ldx::ldx,
	LDX3, 0xB6, 2, 4, AddressingMode::ZeroPageY,	ldx::ldx,
	LDX4, 0xAE, 3, 4, AddressingMode::Absolute,		ldx::ldx,
	LDX5, 0xBE, 3, 4, AddressingMode::AbsoluteY,	ldx::ldx,

	LDY1, 0xA0, 2, 2, AddressingMode::Immediate,	ldy::ldy,
	LDY2, 0xA4, 2, 3, AddressingMode::ZeroPage,		ldy::ldy,
	LDY3, 0xB4, 2, 4, AddressingMode::ZeroPageX,	ldy::ldy,
	LDY4, 0xAC, 3, 4, AddressingMode::Absolute,		ldy::ldy,
	LDY5, 0xBC, 3, 4, AddressingMode::AbsoluteX,	ldy::ldy,

	ASL1, 0x0A, 1, 2, AddressingMode::Immediate,	asl::asl,
	ASL2, 0x06, 2, 5, AddressingMode::ZeroPage,		asl::asl,
	ASL3, 0x16, 2, 6, AddressingMode::ZeroPageX,	asl::asl,
	ASL4, 0x0E, 3, 6, AddressingMode::Absolute,		asl::asl,
	ASL5, 0x1E, 3, 7, AddressingMode::AbsoluteX,	asl::asl,

	AND1, 0x29, 2, 2, AddressingMode::Immediate,	and::and,
	AND2, 0x25, 2, 3, AddressingMode::ZeroPage,		and::and,
	AND3, 0x35, 2, 4, AddressingMode::ZeroPageX,	and::and,
	AND4, 0x2D, 3, 4, AddressingMode::Absolute,		and::and,
	AND5, 0x3D, 3, 4, AddressingMode::AbsoluteX, 	and::and,
	AND6, 0x39, 3, 4, AddressingMode::AbsoluteY, 	and::and,
	AND7, 0x21, 2, 6, AddressingMode::IndirectX, 	and::and,
	AND8, 0x31, 2, 5, AddressingMode::IndirectY, 	and::and,

	EOR1, 0x49, 2, 2, AddressingMode::Immediate,	eor::eor,
	EOR2, 0x45, 2, 3, AddressingMode::ZeroPage,		eor::eor,
	EOR3, 0x55, 2, 4, AddressingMode::ZeroPageX,	eor::eor,
	EOR4, 0x4D, 3, 4, AddressingMode::Absolute,		eor::eor,
	EOR5, 0x5D, 3, 4, AddressingMode::AbsoluteX, 	eor::eor,
	EOR6, 0x59, 3, 4, AddressingMode::AbsoluteY, 	eor::eor,
	EOR7, 0x41, 2, 6, AddressingMode::IndirectX, 	eor::eor,
	EOR8, 0x51, 2, 5, AddressingMode::IndirectY, 	eor::eor,

	STA1, 0x85, 2, 3, AddressingMode::ZeroPage,		sta::sta,
	STA2, 0x95, 2, 4, AddressingMode::ZeroPageX,	sta::sta,
	STA3, 0x8D, 3, 4, AddressingMode::Absolute,		sta::sta,
	STA4, 0x9D, 3, 5, AddressingMode::AbsoluteX, 	sta::sta,
	STA5, 0x99, 3, 5, AddressingMode::AbsoluteY, 	sta::sta,
	STA6, 0x81, 2, 6, AddressingMode::IndirectX, 	sta::sta,
	STA7, 0x91, 2, 6, AddressingMode::IndirectY, 	sta::sta,

	ADC1, 0x69, 2, 2, AddressingMode::Immediate,	adc::adc,
	ADC2, 0x65, 2, 3, AddressingMode::ZeroPage,		adc::adc,
	ADC3, 0x75, 2, 4, AddressingMode::ZeroPageX,	adc::adc,
	ADC4, 0x6D, 3, 4, AddressingMode::Absolute,		adc::adc,
	ADC5, 0x7D, 3, 4, AddressingMode::AbsoluteX, 	adc::adc,
	ADC6, 0x79, 3, 4, AddressingMode::AbsoluteY, 	adc::adc,
	ADC7, 0x61, 2, 6, AddressingMode::IndirectX, 	adc::adc,
	ADC8, 0x71, 2, 5, AddressingMode::IndirectY, 	adc::adc,

	SBC1, 0xE9, 2, 2, AddressingMode::Immediate,	sbc::sbc,
	SBC2, 0xE5, 2, 3, AddressingMode::ZeroPage,		sbc::sbc,
	SBC3, 0xF5, 2, 4, AddressingMode::ZeroPageX,	sbc::sbc,
	SBC4, 0xED, 3, 4, AddressingMode::Absolute,		sbc::sbc,
	SBC5, 0xFD, 3, 4, AddressingMode::AbsoluteX, 	sbc::sbc,
	SBC6, 0xF9, 3, 4, AddressingMode::AbsoluteY, 	sbc::sbc,
	SBC7, 0xE1, 2, 6, AddressingMode::IndirectX, 	sbc::sbc,
	SBC8, 0xF1, 2, 5, AddressingMode::IndirectY, 	sbc::sbc,

	BIT1, 0x24, 2, 3, AddressingMode::ZeroPage, 	bit::bit,
	BIT2, 0x2C, 3, 4, AddressingMode::Absolute, 	bit::bit,

	CMP1, 0xC9, 2, 2, AddressingMode::Immediate,	cmp::cmp,
	CMP2, 0xC5, 2, 3, AddressingMode::ZeroPage,		cmp::cmp,
	CMP3, 0xD5, 2, 4, AddressingMode::ZeroPageX,	cmp::cmp,
	CMP4, 0xCD, 3, 4, AddressingMode::Absolute,		cmp::cmp,
	CMP5, 0xDD, 3, 4, AddressingMode::AbsoluteX,	cmp::cmp,
	CMP6, 0xD9, 3, 4, AddressingMode::AbsoluteY,	cmp::cmp,
	CMP7, 0xC1, 2, 6, AddressingMode::IndirectX,	cmp::cmp,
	CMP8, 0xD1, 2, 5, AddressingMode::IndirectY,	cmp::cmp,

	CPX1, 0xE0, 2, 2, AddressingMode::Immediate,	cpx::cpx,
	CPX2, 0xE4, 2, 3, AddressingMode::ZeroPage,		cpx::cpx,
	CPX3, 0xEC, 3, 4, AddressingMode::Absolute,		cpx::cpx,

	CPY1, 0xC0, 2, 2, AddressingMode::Immediate,	cpy::cpy,
	CPY2, 0xC4, 2, 3, AddressingMode::ZeroPage,		cpy::cpy,
	CPY3, 0xCC, 3, 4, AddressingMode::Absolute,		cpy::cpy,

	JMP1, 0x4C, 3, 3, AddressingMode::Absolute,		jmp::jmp,
	JMP2, 0x6C, 3, 5, AddressingMode::Immediate,	jmp::jmp,

	DEC1, 0xC6, 2, 5, AddressingMode::ZeroPage,		dec::dec,
	DEC2, 0xD6, 2, 6, AddressingMode::ZeroPageX,	dec::dec,
	DEC3, 0xCE, 3, 6, AddressingMode::Absolute,		dec::dec,
	DEC4, 0xDE, 3, 7, AddressingMode::AbsoluteX,	dec::dec,

	INC1, 0xE6, 2, 5, AddressingMode::ZeroPage,		inc::inc,
	INC2, 0xF6, 2, 6, AddressingMode::ZeroPageX,	inc::inc,
	INC3, 0xEE, 3, 6, AddressingMode::Absolute,		inc::inc,
	INC4, 0xFE, 3, 7, AddressingMode::AbsoluteX,	inc::inc,

	STX1, 0x86, 2, 3, AddressingMode::ZeroPage,		stx::stx,
	STX2, 0x96, 2, 4, AddressingMode::ZeroPageY,	stx::stx,
	STX3, 0x8E, 3, 4, AddressingMode::Absolute,		stx::stx,

	STY1, 0x84, 2, 3, AddressingMode::ZeroPage,		sty::sty,
	STY2, 0x94, 2, 4, AddressingMode::ZeroPageX,	sty::sty,
	STY3, 0x8C, 3, 4, AddressingMode::Absolute,		sty::sty,

	LSR_A, 0x4A, 1, 2, AddressingMode::Implied,		lsr::lsr_a,
	LSR_M1, 0x46, 2, 5, AddressingMode::ZeroPage,	lsr::lsr_m,
	LSR_M2, 0x56, 2, 6, AddressingMode::ZeroPageX,	lsr::lsr_m,
	LSR_M3, 0x4E, 3, 6, AddressingMode::Absolute,	lsr::lsr_m,
	LSR_M4, 0x5E, 3, 7, AddressingMode::AbsoluteX,	lsr::lsr_m,

	ORA1, 0x09, 2, 2, AddressingMode::Immediate,	ora::ora,
	ORA2, 0x05, 2, 3, AddressingMode::ZeroPage,		ora::ora,
	ORA3, 0x15, 2, 4, AddressingMode::ZeroPageX,	ora::ora,
	ORA4, 0x0D, 3, 4, AddressingMode::Absolute,		ora::ora,
	ORA5, 0x1D, 3, 4, AddressingMode::AbsoluteX,	ora::ora,
	ORA6, 0x19, 3, 4, AddressingMode::AbsoluteY,	ora::ora,
	ORA7, 0x01, 2, 6, AddressingMode::IndirectX,	ora::ora,
	ORA8, 0x11, 2, 5, AddressingMode::IndirectY,	ora::ora,

	ROL_A, 0x2A, 1, 2, AddressingMode::Implied,		rol::rol_a,
	ROL_M1, 0x26, 2, 5, AddressingMode::ZeroPage,	rol::rol_m,
	ROL_M2, 0x36, 2, 6, AddressingMode::ZeroPageX,	rol::rol_m,
	ROL_M3, 0x2E, 3, 6, AddressingMode::Absolute,	rol::rol_m,
	ROL_M4, 0x3E, 3, 7, AddressingMode::AbsoluteX,	rol::rol_m,

	ROR_A, 0x6A, 1, 2, AddressingMode::Implied,		ror::ror_a,
	ROR_M1, 0x66, 2, 5, AddressingMode::ZeroPage,	ror::ror_m,
	ROR_M2, 0x76, 2, 6, AddressingMode::ZeroPageX,	ror::ror_m,
	ROR_M3, 0x6E, 3, 6, AddressingMode::Absolute,	ror::ror_m,
	ROR_M4, 0x7E, 3, 7, AddressingMode::AbsoluteX,	ror::ror_m
);