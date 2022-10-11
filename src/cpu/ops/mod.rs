use super::memory::*;
use super::AddressingMode;
use super::Cpu;

#[macro_export]
macro_rules! incl {
	( $( $x:ident ),* ) => { $(pub mod $x;)* };
}

incl!(
	// official opcodes                              (brk)
	adc, and, asl, bcc, bcs, beq, bit, bmi, bne, bpl,      bvc, bvs, clc,
	cld, cli, clv, cmp, cpx, cpy, dec, dex, dey, eor, inc, inx, iny, jmp,
	jsr, lda, ldx, ldy, lsr, nop, ora, pha, php, pla, plp, rol, ror, rti,
	rts, sbc, sec, sed, sei, sta, stx, sty, tax, tay, tsx, txa, txs, tya,

	// undocumented opcodes                
	//       (sbc)                         (dop)     (kil)          (nop)
	aac, aax, arr, asr, atx, axa, axs, dcp,      isc,      lar, lax,
	rla, rra,      slo, sre, sxa, sya, xaa, xas
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
			use AddressingMode::*;
			pub const UNSUPPORTED: OpCodeDef = OpCodeDef {
				len: 1,
				cycles: 2,
				mode: AddressingMode::Implied,
				instruction: (|_,_|{}),
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
	TAX,	0xAA, 1, 2, Implied,	Cpu::tax,
	TAY,	0xA8, 1, 2, Implied,	Cpu::tay,
	INX,	0xE8, 1, 2, Implied,	Cpu::inx,
	INY,	0xC8, 1, 2, Implied,	Cpu::iny,
	DEX,	0xCA, 1, 2, Implied,	Cpu::dex,
	DEY,	0x88, 1, 2, Implied,	Cpu::dey,
	TXA,	0x8A, 1, 2, Implied,	Cpu::txa,
	TYA,	0x98, 1, 2, Implied,	Cpu::tya,
	BCC,	0x90, 2, 2, Implied,	Cpu::bcc,
	BCS,	0xB0, 2, 2, Implied,	Cpu::bcs,
	BEQ,	0xF0, 2, 2, Implied,	Cpu::beq,
	BMI,	0x30, 2, 2, Implied,	Cpu::bmi,
	BNE,	0xD0, 2, 2, Implied,	Cpu::bne,
	BPL,	0x10, 2, 2, Implied,	Cpu::bpl,
	BVC,	0x50, 2, 2, Implied,	Cpu::bvc,
	BVS,	0x70, 2, 2, Implied,	Cpu::bvs,
	CLC,	0x18, 1, 2, Implied,	Cpu::clc,
	CLD,	0xD8, 1, 2, Implied,	Cpu::cld,
	CLI,	0x58, 1, 2, Implied,	Cpu::cli,
	CLV,	0xB8, 1, 2, Implied,	Cpu::clv,
	RTS,	0x60, 1, 6, Implied,	Cpu::rts,
	SEC,	0x38, 1, 2, Implied,	Cpu::sec,
	SED,	0xF8, 1, 2, Implied,	Cpu::sed,
	SEI,	0x78, 1, 2, Implied,	Cpu::sei,
	TSX,	0xBA, 1, 2, Implied,	Cpu::tsx,
	TXS,	0x9A, 1, 2, Implied,	Cpu::txs,
	JSR,	0x20, 3, 6, Absolute,	Cpu::jsr,
	PHA,	0x48, 1, 3, Implied,	Cpu::pha,
	PLA,	0x68, 1, 4, Implied,	Cpu::pla,
	PLP,	0x28, 1, 4, IndirectY,	Cpu::plp,
	RTI,	0x40, 1, 6, Implied,	Cpu::rti,
	PHP,	0x08, 1, 3, Implied,	Cpu::php,
	AAR,	0x6B, 2, 2, Immediate,	Cpu::arr,
	ASR,	0x4B, 2, 2, Immediate,	Cpu::asr,
	ATX,	0xAB, 2, 2, Immediate,	Cpu::atx,
	AXS,	0xCB, 2, 2, Immediate,	Cpu::axs,
	LAR,	0xBB, 3, 4, Absolute,	Cpu::lar,
	SXA,	0x9E, 3, 5, AbsoluteY,	Cpu::sxa,
	SYA,	0x9C, 3, 5, AbsoluteX,	Cpu::sya,
	XAA,	0x8B, 2, 2, Immediate,	Cpu::xaa,
	XAS,	0x9B, 3, 5, AbsoluteY,	Cpu::xas,

	NOP1,	0xEA, 1, 2, Implied,	Cpu::nop,
	NOP2,	0x1A, 1, 2, Implied,	Cpu::nop,
	NOP3,	0x3A, 1, 2, Implied,	Cpu::nop,
	NOP4,	0x5A, 1, 2, Implied,	Cpu::nop,
	NOP5,	0x7A, 1, 2, Implied,	Cpu::nop,
	NOP6,	0xDA, 1, 2, Implied,	Cpu::nop,
	NOP7,	0xFA, 1, 2, Implied,	Cpu::nop,

	DOP1,	0x04, 2, 3, ZeroPage,	Cpu::nop,
	DOP2,	0x14, 2, 4, ZeroPageX,	Cpu::nop,
	DOP3,	0x34, 2, 4, ZeroPageX,	Cpu::nop,
	DOP4,	0x44, 2, 3, ZeroPage,	Cpu::nop,
	DOP5,	0x54, 2, 4, ZeroPageX,	Cpu::nop,
	DOP6,	0x64, 2, 3, ZeroPage,	Cpu::nop,
	DOP7,	0x74, 2, 4, ZeroPageX,	Cpu::nop,
	DOP8,	0x80, 2, 2, Immediate,	Cpu::nop,
	DOP9,	0x82, 2, 2, Immediate,	Cpu::nop,
	DOP10,	0x89, 2, 2, Immediate,	Cpu::nop,
	DOP11,	0xC2, 2, 2, Immediate,	Cpu::nop,
	DOP12,	0xD4, 2, 4, ZeroPageX,	Cpu::nop,
	DOP13,	0xE2, 2, 2, Immediate,	Cpu::nop,
	DOP14,	0xF4, 2, 4, ZeroPageX,	Cpu::nop,

	TOP1,	0x0C, 3, 4, Absolute,	Cpu::nop,
	TOP2,	0x1C, 3, 4, AbsoluteX,	Cpu::nop,
	TOP3,	0x3C, 3, 4, AbsoluteX,	Cpu::nop,
	TOP4,	0x5C, 3, 4, AbsoluteX,	Cpu::nop,
	TOP5,	0x7C, 3, 4, AbsoluteX,	Cpu::nop,
	TOP6,	0xDC, 3, 4, AbsoluteX,	Cpu::nop,
	TOP7,	0xFC, 3, 4, AbsoluteX,	Cpu::nop,

	KIL1,	0x02, 0, 0, Implied,	Cpu::nop,
	KIL2,	0x12, 0, 0, Implied,	Cpu::nop,
	KIL3,	0x22, 0, 0, Implied,	Cpu::nop,
	KIL4,	0x32, 0, 0, Implied,	Cpu::nop,
	KIL5,	0x42, 0, 0, Implied,	Cpu::nop,
	KIL6,	0x52, 0, 0, Implied,	Cpu::nop,
	KIL7,	0x62, 0, 0, Implied,	Cpu::nop,
	KIL8,	0x72, 0, 0, Implied,	Cpu::nop,
	KIL9,	0x92, 0, 0, Implied,	Cpu::nop,
	KIL10,	0xB2, 0, 0, Implied,	Cpu::nop,
	KIL11,	0xD2, 0, 0, Implied,	Cpu::nop,
	KIL12,	0xF2, 0, 0, Implied,	Cpu::nop,

	LDA1,	0xA9, 2, 2, Immediate,	Cpu::lda,
	LDA2,	0xA5, 2, 3, ZeroPage,	Cpu::lda,
	LDA3,	0xB5, 2, 4, ZeroPageX,	Cpu::lda,
	LDA4,	0xAD, 3, 4, Absolute,	Cpu::lda,
	LDA5,	0xBD, 3, 4, AbsoluteX,	Cpu::lda,
	LDA6,	0xB9, 3, 4, AbsoluteY,	Cpu::lda,
	LDA7,	0xA1, 2, 6, IndirectX,	Cpu::lda,
	LDA8,	0xB1, 2, 5, IndirectY,	Cpu::lda,

	LDX1,	0xA2, 2, 2, Immediate,	Cpu::ldx,
	LDX2,	0xA6, 2, 3, ZeroPage,	Cpu::ldx,
	LDX3,	0xB6, 2, 4, ZeroPageY,	Cpu::ldx,
	LDX4,	0xAE, 3, 4, Absolute,	Cpu::ldx,
	LDX5,	0xBE, 3, 4, AbsoluteY,	Cpu::ldx,

	LDY1,	0xA0, 2, 2, Immediate,	Cpu::ldy,
	LDY2,	0xA4, 2, 3, ZeroPage,	Cpu::ldy,
	LDY3,	0xB4, 2, 4, ZeroPageX,	Cpu::ldy,
	LDY4,	0xAC, 3, 4, Absolute,	Cpu::ldy,
	LDY5,	0xBC, 3, 4, AbsoluteX,	Cpu::ldy,

	ASL1,	0x0A, 1, 2, Immediate,	Cpu::asl,
	ASL2,	0x06, 2, 5, ZeroPage,	Cpu::asl,
	ASL3,	0x16, 2, 6, ZeroPageX,	Cpu::asl,
	ASL4,	0x0E, 3, 6, Absolute,	Cpu::asl,
	ASL5,	0x1E, 3, 7, AbsoluteX,	Cpu::asl,

	AND1,	0x29, 2, 2, Immediate,	Cpu::and,
	AND2,	0x25, 2, 3, ZeroPage,	Cpu::and,
	AND3,	0x35, 2, 4, ZeroPageX,	Cpu::and,
	AND4,	0x2D, 3, 4, Absolute,	Cpu::and,
	AND5,	0x3D, 3, 4, AbsoluteX, 	Cpu::and,
	AND6,	0x39, 3, 4, AbsoluteY, 	Cpu::and,
	AND7,	0x21, 2, 6, IndirectX, 	Cpu::and,
	AND8,	0x31, 2, 5, IndirectY, 	Cpu::and,

	EOR1,	0x49, 2, 2, Immediate,	Cpu::eor,
	EOR2,	0x45, 2, 3, ZeroPage,	Cpu::eor,
	EOR3,	0x55, 2, 4, ZeroPageX,	Cpu::eor,
	EOR4,	0x4D, 3, 4, Absolute,	Cpu::eor,
	EOR5,	0x5D, 3, 4, AbsoluteX, 	Cpu::eor,
	EOR6,	0x59, 3, 4, AbsoluteY, 	Cpu::eor,
	EOR7,	0x41, 2, 6, IndirectX, 	Cpu::eor,
	EOR8,	0x51, 2, 5, IndirectY, 	Cpu::eor,

	STA1,	0x85, 2, 3, ZeroPage,	Cpu::sta,
	STA2,	0x95, 2, 4, ZeroPageX,	Cpu::sta,
	STA3,	0x8D, 3, 4, Absolute,	Cpu::sta,
	STA4,	0x9D, 3, 5, AbsoluteX, 	Cpu::sta,
	STA5,	0x99, 3, 5, AbsoluteY, 	Cpu::sta,
	STA6,	0x81, 2, 6, IndirectX, 	Cpu::sta,
	STA7,	0x91, 2, 6, IndirectY, 	Cpu::sta,

	ADC1,	0x69, 2, 2, Immediate,	Cpu::adc,
	ADC2,	0x65, 2, 3, ZeroPage,	Cpu::adc,
	ADC3,	0x75, 2, 4, ZeroPageX,	Cpu::adc,
	ADC4,	0x6D, 3, 4, Absolute,	Cpu::adc,
	ADC5,	0x7D, 3, 4, AbsoluteX, 	Cpu::adc,
	ADC6,	0x79, 3, 4, AbsoluteY, 	Cpu::adc,
	ADC7,	0x61, 2, 6, IndirectX, 	Cpu::adc,
	ADC8,	0x71, 2, 5, IndirectY, 	Cpu::adc,

	SBC1,	0xE9, 2, 2, Immediate,	Cpu::sbc,
	SBC2,	0xE5, 2, 3, ZeroPage,	Cpu::sbc,
	SBC3,	0xF5, 2, 4, ZeroPageX,	Cpu::sbc,
	SBC4,	0xED, 3, 4, Absolute,	Cpu::sbc,
	SBC5,	0xFD, 3, 4, AbsoluteX, 	Cpu::sbc,
	SBC6,	0xF9, 3, 4, AbsoluteY, 	Cpu::sbc,
	SBC7,	0xE1, 2, 6, IndirectX, 	Cpu::sbc,
	SBC8,	0xF1, 2, 5, IndirectY, 	Cpu::sbc,
	SBC9,	0xEB, 2, 2, Immediate,	Cpu::sbc,

	BIT1,	0x24, 2, 3, ZeroPage, 	Cpu::bit,
	BIT2,	0x2C, 3, 4, Absolute, 	Cpu::bit,

	CMP1,	0xC9, 2, 2, Immediate,	Cpu::cmp,
	CMP2,	0xC5, 2, 3, ZeroPage,	Cpu::cmp,
	CMP3,	0xD5, 2, 4, ZeroPageX,	Cpu::cmp,
	CMP4,	0xCD, 3, 4, Absolute,	Cpu::cmp,
	CMP5,	0xDD, 3, 4, AbsoluteX,	Cpu::cmp,
	CMP6,	0xD9, 3, 4, AbsoluteY,	Cpu::cmp,
	CMP7,	0xC1, 2, 6, IndirectX,	Cpu::cmp,
	CMP8,	0xD1, 2, 5, IndirectY,	Cpu::cmp,

	CPX1,	0xE0, 2, 2, Immediate,	Cpu::cpx,
	CPX2,	0xE4, 2, 3, ZeroPage,	Cpu::cpx,
	CPX3,	0xEC, 3, 4, Absolute,	Cpu::cpx,

	CPY1,	0xC0, 2, 2, Immediate,	Cpu::cpy,
	CPY2,	0xC4, 2, 3, ZeroPage,	Cpu::cpy,
	CPY3,	0xCC, 3, 4, Absolute,	Cpu::cpy,

	JMP1,	0x4C, 3, 3, Absolute,	Cpu::jmp_absolute,
	JMP2,	0x6C, 3, 5, Immediate,	Cpu::jmp,

	DEC1,	0xC6, 2, 5, ZeroPage,	Cpu::dec,
	DEC2,	0xD6, 2, 6, ZeroPageX,	Cpu::dec,
	DEC3,	0xCE, 3, 6, Absolute,	Cpu::dec,
	DEC4,	0xDE, 3, 7, AbsoluteX,	Cpu::dec,

	INC1,	0xE6, 2, 5, ZeroPage,	Cpu::inc,
	INC2,	0xF6, 2, 6, ZeroPageX,	Cpu::inc,
	INC3,	0xEE, 3, 6, Absolute,	Cpu::inc,
	INC4,	0xFE, 3, 7, AbsoluteX,	Cpu::inc,

	STX1,	0x86, 2, 3, ZeroPage,	Cpu::stx,
	STX2,	0x96, 2, 4, ZeroPageY,	Cpu::stx,
	STX3,	0x8E, 3, 4, Absolute,	Cpu::stx,

	STY1,	0x84, 2, 3, ZeroPage,	Cpu::sty,
	STY2,	0x94, 2, 4, ZeroPageX,	Cpu::sty,
	STY3,	0x8C, 3, 4, Absolute,	Cpu::sty,

	LSR_A,	0x4A, 1, 2, Implied,	Cpu::lsr_a,
	LSR_M1,	0x46, 2, 5, ZeroPage,	Cpu::lsr_m,
	LSR_M2,	0x56, 2, 6, ZeroPageX,	Cpu::lsr_m,
	LSR_M3,	0x4E, 3, 6, Absolute,	Cpu::lsr_m,
	LSR_M4,	0x5E, 3, 7, AbsoluteX,	Cpu::lsr_m,

	ORA1,	0x09, 2, 2, Immediate,	Cpu::ora,
	ORA2,	0x05, 2, 3, ZeroPage,	Cpu::ora,
	ORA3,	0x15, 2, 4, ZeroPageX,	Cpu::ora,
	ORA4,	0x0D, 3, 4, Absolute,	Cpu::ora,
	ORA5,	0x1D, 3, 4, AbsoluteX,	Cpu::ora,
	ORA6,	0x19, 3, 4, AbsoluteY,	Cpu::ora,
	ORA7,	0x01, 2, 6, IndirectX,	Cpu::ora,
	ORA8,	0x11, 2, 5, IndirectY,	Cpu::ora,

	ROL_A,	0x2A, 1, 2, Implied,	Cpu::rol_a,
	ROL_M1,	0x26, 2, 5, ZeroPage,	Cpu::rol_m,
	ROL_M2,	0x36, 2, 6, ZeroPageX,	Cpu::rol_m,
	ROL_M3,	0x2E, 3, 6, Absolute,	Cpu::rol_m,
	ROL_M4,	0x3E, 3, 7, AbsoluteX,	Cpu::rol_m,

	ROR_A,	0x6A, 1, 2, Implied,	Cpu::ror_a,
	ROR_M1,	0x66, 2, 5, ZeroPage,	Cpu::ror_m,
	ROR_M2,	0x76, 2, 6, ZeroPageX,	Cpu::ror_m,
	ROR_M3,	0x6E, 3, 6, Absolute,	Cpu::ror_m,
	ROR_M4,	0x7E, 3, 7, AbsoluteX,	Cpu::ror_m,

	AAC1,	0x0B, 2, 2, Immediate, 	Cpu::aac,
	AAC2,	0x2B, 2, 2, Immediate, 	Cpu::aac,

	AAX1,	0x87, 2, 3, ZeroPage,	Cpu::aax,
	AAX2,	0x97, 2, 4, ZeroPageY,	Cpu::aax,
	AAX3,	0x83, 2, 6, IndirectX,	Cpu::aax,
	AAX4,	0x8F, 3, 4, Absolute,	Cpu::aax,

	AXA1,	0x9F, 3, 5, AbsoluteY,	Cpu::axa_ab,
	AXA2,	0x93, 2, 6, IndirectY,	Cpu::axa_in,

	DCP1,	0xC7, 2, 5, ZeroPage,	Cpu::dcp,
	DCP2,	0xD7, 2, 6, ZeroPageX,	Cpu::dcp,
	DCP3,	0xCF, 3, 6, Absolute,	Cpu::dcp,
	DCP4,	0xDF, 3, 7, AbsoluteX,	Cpu::dcp,
	DCP5,	0xDB, 3, 7, AbsoluteY,	Cpu::dcp,
	DCP6,	0xC3, 2, 8, IndirectX,	Cpu::dcp,
	DCP7,	0xD3, 2, 8, IndirectY,	Cpu::dcp,

	ISC1,	0xE7, 2, 5, ZeroPage,	Cpu::isc,
	ISC2,	0xF7, 2, 6, ZeroPageX,	Cpu::isc,
	ISC3,	0xEF, 3, 6, Absolute,	Cpu::isc,
	ISC4,	0xFF, 3, 7, AbsoluteX,	Cpu::isc,
	ISC5,	0xFB, 3, 7, AbsoluteY,	Cpu::isc,
	ISC6,	0xE3, 2, 8, IndirectX,	Cpu::isc,
	ISC7,	0xF3, 2, 8, IndirectY,	Cpu::isc,

	LAX1,	0xA7, 2, 3, ZeroPage,	Cpu::lax,
	LAX2,	0xB7, 2, 4, ZeroPageY,	Cpu::lax,
	LAX3,	0xAF, 3, 4, Absolute,	Cpu::lax,
	LAX4,	0xBF, 3, 4, AbsoluteY,	Cpu::lax,
	LAX5,	0xA3, 2, 6, IndirectX,	Cpu::lax,
	LAX6,	0xB3, 2, 5, IndirectY,	Cpu::lax,

	RLA1,	0x27, 2, 5, ZeroPage,	Cpu::rla,
	RLA2,	0x37, 2, 6, ZeroPageX,	Cpu::rla,
	RLA3,	0x2F, 3, 6, Absolute,	Cpu::rla,
	RLA4,	0x3F, 3, 7, AbsoluteX,	Cpu::rla,
	RLA5,	0x3B, 3, 7, AbsoluteY,	Cpu::rla,
	RLA6,	0x23, 2, 8, IndirectX,	Cpu::rla,
	RLA7,	0x33, 2, 8, IndirectY,	Cpu::rla,

	RRA1,	0x67, 2, 5, ZeroPage,	Cpu::rra,
	RRA2,	0x77, 2, 6, ZeroPageX,	Cpu::rra,
	RRA3,	0x6F, 3, 6, Absolute,	Cpu::rra,
	RRA4,	0x7F, 3, 7, AbsoluteX,	Cpu::rra,
	RRA5,	0x7B, 3, 7, AbsoluteY,	Cpu::rra,
	RRA6,	0x63, 2, 8, IndirectX,	Cpu::rra,
	RRA7,	0x73, 2, 8, IndirectY,	Cpu::rra,

	SLO1,	0x07, 2, 5, ZeroPage,	Cpu::slo,
	SLO2,	0x17, 2, 6, ZeroPageX,	Cpu::slo,
	SLO3,	0x0F, 3, 6, Absolute,	Cpu::slo,
	SLO4,	0x1F, 3, 7, AbsoluteX,	Cpu::slo,
	SLO5,	0x1B, 3, 7, AbsoluteY,	Cpu::slo,
	SLO6,	0x03, 2, 8, IndirectX,	Cpu::slo,
	SLO7,	0x13, 2, 8, IndirectY,	Cpu::slo,

	SRE1,	0x47, 2, 5, ZeroPage,	Cpu::sre,
	SRE2,	0x57, 2, 6, ZeroPageX,	Cpu::sre,
	SRE3,	0x4F, 3, 6, Absolute,	Cpu::sre,
	SRE4,	0x5F, 3, 7, AbsoluteX,	Cpu::sre,
	SRE5,	0x5B, 3, 7, AbsoluteY,	Cpu::sre,
	SRE6,	0x43, 2, 8, IndirectX,	Cpu::sre,
	SRE7,	0x53, 2, 8, IndirectY,	Cpu::sre
);