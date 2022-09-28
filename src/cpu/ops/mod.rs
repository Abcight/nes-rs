use super::memory::*;
use super::AddressingMode;
use super::Cpu;

#[macro_export]
macro_rules! incl {
	( $( $x:ident ),* ) => { $(pub mod $x; use $x::*;)* };
}

incl!(
	// official opcodes                              (brk)
	adc, and, asl, bcc, bcs, beq, bit, bmi, bne, bpl,      bvc, bvs, clc,
	cld, cli, clv, cmp, cpx, cpy, dec, dex, dey, eor, inc, inx, iny, jmp,
	jsr, lda, ldx, ldy, lsr, nop, ora, pha, php, pla, plp, rol, ror, rti,
	rts, sbc, sec, sed, sei, sta, stx, sty, tax, tay, tsx, txa, txs, tya,

	// undocumented opcodes                (dop)
	aac, aax, arr, asr, atx, axa, axs, dcp,      isc, lar
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
	TAX,	0xAA, 1, 2, Implied,	tax,
	TAY,	0xA8, 1, 2, Implied,	tay,
	INX,	0xE8, 1, 2, Implied,	inx,
	INY,	0xC8, 1, 2, Implied,	iny,
	DEX,	0xCA, 1, 2, Implied,	dex,
	DEY,	0x88, 1, 2, Implied,	dey,
	TXA,	0x8A, 1, 2, Implied,	txa,
	TYA,	0x98, 1, 2, Implied,	tya,
	BCC,	0x90, 2, 2, Implied,	bcc,
	BCS,	0xB0, 2, 2, Implied,	bcs,
	BEQ,	0xF0, 2, 2, Implied,	beq,
	BMI,	0x30, 2, 2, Implied,	bmi,
	BNE,	0xD0, 2, 2, Implied,	bne,
	BPL,	0x10, 2, 2, Implied,	bpl,
	BVC,	0x50, 2, 2, Implied,	bvc,
	BVS,	0x70, 2, 2, Implied,	bvs,
	CLC,	0x18, 1, 2, Implied,	clc,
	CLD,	0xD8, 1, 2, Implied,	cld,
	CLI,	0x58, 1, 2, Implied,	cli,
	CLV,	0xB8, 1, 2, Implied,	clv,
	RTS,	0x60, 1, 6, Implied,	rts,
	SEC,	0x38, 1, 2, Implied,	sec,
	SED,	0xF8, 1, 2, Implied,	sed,
	SEI,	0x78, 1, 2, Implied,	sei,
	TSX,	0xBA, 1, 2, Implied,	tsx,
	TXS,	0x9A, 1, 2, Implied,	txs,
	JSR,	0x20, 3, 6, Absolute,	jsr,
	PHA,	0x48, 1, 3, Implied,	pha,
	PLA,	0x68, 1, 4, Implied,	pla,
	PLP,	0x28, 1, 4, IndirectY,	plp,
	RTI,	0x40, 1, 6, Implied,	rti,
	PHP,	0x08, 1, 3, Implied,	php,
	AAR,	0x6B, 2, 2, Immediate,	arr,
	ASR,	0x4B, 2, 2, Immediate,	asr,
	ATX,	0xAB, 2, 2, Immediate,	atx,
	AXS,	0xCB, 2, 2, Immediate,	axs,
	LAR,	0xBB, 3, 4, Absolute,	lar,

	NOP1,	0xEA, 1, 2, Implied,	nop,
	NOP2,	0x1A, 1, 2, Implied,	nop,
	NOP3,	0x3A, 1, 2, Implied,	nop,
	NOP4,	0x5A, 1, 2, Implied,	nop,
	NOP5,	0x7A, 1, 2, Implied,	nop,
	NOP6,	0xDA, 1, 2, Implied,	nop,
	NOP7,	0xFA, 1, 2, Implied,	nop,

	DOP1,	0x04, 2, 3, ZeroPage,	nop,
	DOP2,	0x14, 2, 4, ZeroPageX,	nop,
	DOP3,	0x34, 2, 4, ZeroPageX,	nop,
	DOP4,	0x44, 2, 3, ZeroPage,	nop,
	DOP5,	0x54, 2, 4, ZeroPageX,	nop,
	DOP6,	0x64, 2, 3, ZeroPage,	nop,
	DOP7,	0x74, 2, 4, ZeroPageX,	nop,
	DOP8,	0x80, 2, 2, Immediate,	nop,
	DOP9,	0x82, 2, 2, Immediate,	nop,
	DOP10,	0x89, 2, 2, Immediate,	nop,
	DOP11,	0xC2, 2, 2, Immediate,	nop,
	DOP12,	0xD4, 2, 4, ZeroPageX,	nop,
	DOP13,	0xE2, 2, 2, Immediate,	nop,
	DOP14,	0xF4, 2, 4, ZeroPageX,	nop,

	TOP1,	0x0C, 3, 4, Absolute,	nop,
	TOP2,	0x1C, 3, 4, AbsoluteX,	nop,
	TOP3,	0x3C, 3, 4, AbsoluteX,	nop,
	TOP4,	0x5C, 3, 4, AbsoluteX,	nop,
	TOP5,	0x7C, 3, 4, AbsoluteX,	nop,
	TOP6,	0xDC, 3, 4, AbsoluteX,	nop,
	TOP7,	0xFC, 3, 4, AbsoluteX,	nop,

	KIL1,	0x02, 0, 0, Implied,	nop,
	KIL2,	0x12, 0, 0, Implied,	nop,
	KIL3,	0x22, 0, 0, Implied,	nop,
	KIL4,	0x32, 0, 0, Implied,	nop,
	KIL5,	0x42, 0, 0, Implied,	nop,
	KIL6,	0x52, 0, 0, Implied,	nop,
	KIL7,	0x62, 0, 0, Implied,	nop,
	KIL8,	0x72, 0, 0, Implied,	nop,
	KIL9,	0x92, 0, 0, Implied,	nop,
	KIL10,	0xB2, 0, 0, Implied,	nop,
	KIL11,	0xD2, 0, 0, Implied,	nop,
	KIL12,	0xF2, 0, 0, Implied,	nop,

	LDA1,	0xA9, 2, 2, Immediate,	lda,
	LDA2,	0xA5, 2, 3, ZeroPage,	lda,
	LDA3,	0xB5, 2, 4, ZeroPageX,	lda,
	LDA4,	0xAD, 3, 4, Absolute,	lda,
	LDA5,	0xBD, 3, 4, AbsoluteX,	lda,
	LDA6,	0xB9, 3, 4, AbsoluteY,	lda,
	LDA7,	0xA1, 2, 6, IndirectX,	lda,
	LDA8,	0xB1, 2, 5, IndirectY,	lda,

	LDX1,	0xA2, 2, 2, Immediate,	ldx,
	LDX2,	0xA6, 2, 3, ZeroPage,	ldx,
	LDX3,	0xB6, 2, 4, ZeroPageY,	ldx,
	LDX4,	0xAE, 3, 4, Absolute,	ldx,
	LDX5,	0xBE, 3, 4, AbsoluteY,	ldx,

	LDY1,	0xA0, 2, 2, Immediate,	ldy,
	LDY2,	0xA4, 2, 3, ZeroPage,	ldy,
	LDY3,	0xB4, 2, 4, ZeroPageX,	ldy,
	LDY4,	0xAC, 3, 4, Absolute,	ldy,
	LDY5,	0xBC, 3, 4, AbsoluteX,	ldy,

	ASL1,	0x0A, 1, 2, Immediate,	asl,
	ASL2,	0x06, 2, 5, ZeroPage,	asl,
	ASL3,	0x16, 2, 6, ZeroPageX,	asl,
	ASL4,	0x0E, 3, 6, Absolute,	asl,
	ASL5,	0x1E, 3, 7, AbsoluteX,	asl,

	AND1,	0x29, 2, 2, Immediate,	and,
	AND2,	0x25, 2, 3, ZeroPage,	and,
	AND3,	0x35, 2, 4, ZeroPageX,	and,
	AND4,	0x2D, 3, 4, Absolute,	and,
	AND5,	0x3D, 3, 4, AbsoluteX, 	and,
	AND6,	0x39, 3, 4, AbsoluteY, 	and,
	AND7,	0x21, 2, 6, IndirectX, 	and,
	AND8,	0x31, 2, 5, IndirectY, 	and,

	EOR1,	0x49, 2, 2, Immediate,	eor,
	EOR2,	0x45, 2, 3, ZeroPage,	eor,
	EOR3,	0x55, 2, 4, ZeroPageX,	eor,
	EOR4,	0x4D, 3, 4, Absolute,	eor,
	EOR5,	0x5D, 3, 4, AbsoluteX, 	eor,
	EOR6,	0x59, 3, 4, AbsoluteY, 	eor,
	EOR7,	0x41, 2, 6, IndirectX, 	eor,
	EOR8,	0x51, 2, 5, IndirectY, 	eor,

	STA1,	0x85, 2, 3, ZeroPage,	sta,
	STA2,	0x95, 2, 4, ZeroPageX,	sta,
	STA3,	0x8D, 3, 4, Absolute,	sta,
	STA4,	0x9D, 3, 5, AbsoluteX, 	sta,
	STA5,	0x99, 3, 5, AbsoluteY, 	sta,
	STA6,	0x81, 2, 6, IndirectX, 	sta,
	STA7,	0x91, 2, 6, IndirectY, 	sta,

	ADC1,	0x69, 2, 2, Immediate,	adc,
	ADC2,	0x65, 2, 3, ZeroPage,	adc,
	ADC3,	0x75, 2, 4, ZeroPageX,	adc,
	ADC4,	0x6D, 3, 4, Absolute,	adc,
	ADC5,	0x7D, 3, 4, AbsoluteX, 	adc,
	ADC6,	0x79, 3, 4, AbsoluteY, 	adc,
	ADC7,	0x61, 2, 6, IndirectX, 	adc,
	ADC8,	0x71, 2, 5, IndirectY, 	adc,

	SBC1,	0xE9, 2, 2, Immediate,	sbc,
	SBC2,	0xE5, 2, 3, ZeroPage,	sbc,
	SBC3,	0xF5, 2, 4, ZeroPageX,	sbc,
	SBC4,	0xED, 3, 4, Absolute,	sbc,
	SBC5,	0xFD, 3, 4, AbsoluteX, 	sbc,
	SBC6,	0xF9, 3, 4, AbsoluteY, 	sbc,
	SBC7,	0xE1, 2, 6, IndirectX, 	sbc,
	SBC8,	0xF1, 2, 5, IndirectY, 	sbc,
	SBC9,	0xEB, 2, 2, Immediate,	sbc,

	BIT1,	0x24, 2, 3, ZeroPage, 	bit,
	BIT2,	0x2C, 3, 4, Absolute, 	bit,

	CMP1,	0xC9, 2, 2, Immediate,	cmp,
	CMP2,	0xC5, 2, 3, ZeroPage,	cmp,
	CMP3,	0xD5, 2, 4, ZeroPageX,	cmp,
	CMP4,	0xCD, 3, 4, Absolute,	cmp,
	CMP5,	0xDD, 3, 4, AbsoluteX,	cmp,
	CMP6,	0xD9, 3, 4, AbsoluteY,	cmp,
	CMP7,	0xC1, 2, 6, IndirectX,	cmp,
	CMP8,	0xD1, 2, 5, IndirectY,	cmp,

	CPX1,	0xE0, 2, 2, Immediate,	cpx,
	CPX2,	0xE4, 2, 3, ZeroPage,	cpx,
	CPX3,	0xEC, 3, 4, Absolute,	cpx,

	CPY1,	0xC0, 2, 2, Immediate,	cpy,
	CPY2,	0xC4, 2, 3, ZeroPage,	cpy,
	CPY3,	0xCC, 3, 4, Absolute,	cpy,

	JMP1,	0x4C, 3, 3, Absolute,	jmp,
	JMP2,	0x6C, 3, 5, Immediate,	jmp,

	DEC1,	0xC6, 2, 5, ZeroPage,	dec,
	DEC2,	0xD6, 2, 6, ZeroPageX,	dec,
	DEC3,	0xCE, 3, 6, Absolute,	dec,
	DEC4,	0xDE, 3, 7, AbsoluteX,	dec,

	INC1,	0xE6, 2, 5, ZeroPage,	inc,
	INC2,	0xF6, 2, 6, ZeroPageX,	inc,
	INC3,	0xEE, 3, 6, Absolute,	inc,
	INC4,	0xFE, 3, 7, AbsoluteX,	inc,

	STX1,	0x86, 2, 3, ZeroPage,	stx,
	STX2,	0x96, 2, 4, ZeroPageY,	stx,
	STX3,	0x8E, 3, 4, Absolute,	stx,

	STY1,	0x84, 2, 3, ZeroPage,	sty,
	STY2,	0x94, 2, 4, ZeroPageX,	sty,
	STY3,	0x8C, 3, 4, Absolute,	sty,

	LSR_A,	0x4A, 1, 2, Implied,	lsr_a,
	LSR_M1,	0x46, 2, 5, ZeroPage,	lsr_m,
	LSR_M2,	0x56, 2, 6, ZeroPageX,	lsr_m,
	LSR_M3,	0x4E, 3, 6, Absolute,	lsr_m,
	LSR_M4,	0x5E, 3, 7, AbsoluteX,	lsr_m,

	ORA1,	0x09, 2, 2, Immediate,	ora,
	ORA2,	0x05, 2, 3, ZeroPage,	ora,
	ORA3,	0x15, 2, 4, ZeroPageX,	ora,
	ORA4,	0x0D, 3, 4, Absolute,	ora,
	ORA5,	0x1D, 3, 4, AbsoluteX,	ora,
	ORA6,	0x19, 3, 4, AbsoluteY,	ora,
	ORA7,	0x01, 2, 6, IndirectX,	ora,
	ORA8,	0x11, 2, 5, IndirectY,	ora,

	ROL_A,	0x2A, 1, 2, Implied,	rol_a,
	ROL_M1,	0x26, 2, 5, ZeroPage,	rol_m,
	ROL_M2,	0x36, 2, 6, ZeroPageX,	rol_m,
	ROL_M3,	0x2E, 3, 6, Absolute,	rol_m,
	ROL_M4,	0x3E, 3, 7, AbsoluteX,	rol_m,

	ROR_A,	0x6A, 1, 2, Implied,	ror_a,
	ROR_M1,	0x66, 2, 5, ZeroPage,	ror_m,
	ROR_M2,	0x76, 2, 6, ZeroPageX,	ror_m,
	ROR_M3,	0x6E, 3, 6, Absolute,	ror_m,
	ROR_M4,	0x7E, 3, 7, AbsoluteX,	ror_m,

	AAC1,	0x0B, 2, 2, Immediate, 	aac,
	AAC2,	0x2B, 2, 2, Immediate, 	aac,

	AAX1,	0x87, 2, 3, ZeroPage,	aax,
	AAX2,	0x97, 2, 4, ZeroPageY,	aax,
	AAX3,	0x83, 2, 6, IndirectX,	aax,
	AAX4,	0x8F, 3, 4, Absolute,	aax,

	AXA1,	0x9F, 3, 5, AbsoluteY,	axa_ab,
	AXA2,	0x93, 2, 6, IndirectY,	axa_in,

	DCP1,	0xC7, 2, 5, ZeroPage,	dcp,
	DCP2,	0xD7, 2, 6, ZeroPageX,	dcp,
	DCP3,	0xCF, 3, 6, Absolute,	dcp,
	DCP4,	0xDF, 3, 7, AbsoluteX,	dcp,
	DCP5,	0xDB, 3, 7, AbsoluteY,	dcp,
	DCP6,	0xC3, 2, 8, IndirectX,	dcp,
	DCP7,	0xD3, 2, 8, IndirectY,	dcp,

	ISC1,	0xE7, 2, 5, ZeroPage,	isc,
	ISC2,	0xF7, 2, 6, ZeroPageX,	isc,
	ISC3,	0xEF, 3, 6, Absolute,	isc,
	ISC4,	0xFF, 3, 7, AbsoluteX,	isc,
	ISC5,	0xFB, 3, 7, AbsoluteY,	isc,
	ISC6,	0xE3, 2, 8, IndirectX,	isc,
	ISC7,	0xF3, 2, 8, IndirectY,	isc
);