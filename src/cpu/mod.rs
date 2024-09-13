pub mod ops;

mod status;
use status::*;

pub use super::memory::Memory;
pub use super::bus::Bus;

pub const STACK: u16 = 0x0100;
pub const DEFAULT_STACK: u8 = 0xfd;

pub enum AddressingMode {
	Immediate,
	ZeroPage,
	ZeroPageX,
	ZeroPageY,
	Absolute,
	AbsoluteX,
	AbsoluteY,
	IndirectX,
	IndirectY,
	Implied,
}

pub struct Cpu {
	pub register_a: u8,
	pub register_x: u8,
	pub register_y: u8,
	pub status: CpuStatus,
	pub program_counter: u16,
	pub stack_pointer: u8,
	pub bus: Bus,
}

impl Cpu {
	pub fn new() -> Self {
		Cpu {
			register_a: 0,
			register_x: 0,
			register_y: 0,
			status: CpuStatus(0),
			program_counter: 0,
			stack_pointer: DEFAULT_STACK,
			bus: Bus::new()
		}
	}

	pub fn interpret(&mut self, program: Vec<u8>) {
		self.load(program);
		self.reset();
		self.run();
	}

	pub fn run(&mut self) {
		let mut opcode = self.read(self.program_counter);
		while opcode != 0x00 {
			self.program_counter += 1;
			let def = ops::get_instruction_def(opcode);
			def.execute(self);
			self.program_counter += (def.len - 1) as u16;
			opcode = self.read(self.program_counter);
		}
	}

	pub fn load(&mut self, program: Vec<u8>) {
		for i in 0..(program.len() as u16) {
			self.write(0x0600 + i, program[i as usize]);
		}
		self.write_u16(0xFFFC, 0x0600);
	}

	pub fn reset(&mut self) {
		self.register_a = 0;
		self.register_x = 0;
		self.register_y = 0;
		self.status = CpuStatus(0b100100);
		self.stack_pointer = DEFAULT_STACK;
		self.program_counter = 0x0600;
	}

	pub fn branch_if(&mut self, condition: bool) {
		if condition {
			let jmp = self.read(self.program_counter) as i8;
			self.program_counter =
				self.program_counter
				.wrapping_add(1)
				.wrapping_add(jmp as u16);
		}
	}

	fn pop(&mut self) -> u8 {
		self.stack_pointer = self.stack_pointer.wrapping_add(1);
		self.read((STACK as u16) + self.stack_pointer as u16)
	}

	fn push(&mut self, data: u8) {
		self.write((STACK as u16) + self.stack_pointer as u16, data);
		self.stack_pointer = self.stack_pointer.wrapping_sub(1)
	}

	fn push_u16(&mut self, data: u16) {
		let hi = (data >> 8) as u8;
		let lo = (data & 0xff) as u8;
		self.push(hi);
		self.push(lo);
	}

	fn pop_u16(&mut self) -> u16 {
		let lo = self.pop() as u16;
		let hi = self.pop() as u16;
		hi << 8 | lo
	}

	fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
		match mode {
			AddressingMode::Immediate => self.program_counter,
			AddressingMode::ZeroPage => self.read(self.program_counter) as u16,
			AddressingMode::Absolute => self.read_u16(self.program_counter),
			AddressingMode::ZeroPageX => {
				let pos = self.read(self.program_counter);
				pos.wrapping_add(self.register_x) as u16
			}
			AddressingMode::ZeroPageY => {
				let pos = self.read(self.program_counter);
				pos.wrapping_add(self.register_y) as u16
			}
			AddressingMode::AbsoluteX => {
				let base = self.read_u16(self.program_counter);
				base.wrapping_add(self.register_x as u16)
			}
			AddressingMode::AbsoluteY => {
				let base = self.read_u16(self.program_counter);
				base.wrapping_add(self.register_y as u16)
			}
			AddressingMode::IndirectX => {
				let base = self.read(self.program_counter);
				let ptr: u8 = (base as u8).wrapping_add(self.register_x);
				let lo = self.read(ptr as u16);
				let hi = self.read(ptr.wrapping_add(1) as u16);
				(hi as u16) << 8 | (lo as u16)
			}
			AddressingMode::IndirectY => {
				let base = self.read(self.program_counter);
				let lo = self.read(base as u16);
				let hi = self.read((base as u8).wrapping_add(1) as u16);
				let deref_base = (hi as u16) << 8 | (lo as u16);
				deref_base.wrapping_add(self.register_y as u16)
			}
			AddressingMode::Implied => {
				panic!("Unsupported address mode");
			}
		}
	}

	fn set_zero_neg_flags(&mut self, result: u8) {
		self.status.set_zero(result == 0);
		self.status.set_negative(result & 0b1000_0000 != 0);
	}
}

impl Memory for Cpu {
	fn read(&self, addr: u16) -> u8 {
		self.bus.read(addr)
	}

	fn write(&mut self, addr: u16, data: u8) {
		self.bus.write(addr, data);
	}
	fn read_u16(&self, pos: u16) -> u16 {
		self.bus.read_u16(pos)
	}

	fn write_u16(&mut self, pos: u16, data: u16) {
		self.bus.write_u16(pos, data);
	}
}

// OPCODES
pub enum OpCode {
	AAC = 0x0B,
	AAX = 0x87,
	ADC = 0x69,
	AND = 0x29,
	ARR = 0x6B,
	ASL = 0x0A,
	ASR = 0x4B,
	TXA = 0x8A,
	TXS = 0x9A,
	TYA = 0x98,
	XAS = 0x00, // TODO: Fix this
	XAA = 0x8B,
	TSX = 0xBA,
	TAY = 0xA8,
	TAX = 0xAA,
	SYA = 0x9C,
	SXA = 0x9E,
	STY = 0x84,
	STX = 0x86,
	STA = 0x85,
	SRE = 0x47,
	SLO = 0x07,
	SEI = 0x78,
	SED = 0xF8,
	SEC = 0x38,
	SBC = 0xE9,
	RTS = 0x60,
	RTI = 0x40,
	RRA = 0x63,
	ROR = 0x6A,
	ROL = 0x2A,
	RLA = 0x27,
	PLP = 0x28,
	PLA = 0x68,
	PHP = 0x08,
	PHA = 0x48,
	ORA = 0x09,
	NOP = 0xEA,
	LSR = 0x4A,
	LDY = 0xA0,
	LDX = 0xA2,
	LDA = 0xA9,
	LAX = 0xA7,
	LAR = 0xBB,
	JSR = 0x20,
	JMP = 0x4C,
	INY = 0xC8,
	INX = 0xE8,
	INC = 0xE6,
	EOR = 0x49,
	DEY = 0x88,
	DEX = 0xCA,
	DEC = 0xC6,
	DCP = 0xC3,
	CPY = 0xC0,
	CPX = 0xE0,
	CMP = 0xC9,
	CLV = 0xB8,
	CLI = 0x58,
	CLD = 0xD8,
	CLC = 0x18,
	BVS = 0x70,
	BVC = 0x50,
	BPL = 0x10,
	BNE = 0xD0,
	BMI = 0x30,
	BIT = 0x24,
	BEQ = 0xF0,
	BCS = 0xB0,
	BCC = 0x90,
	AXS = 0xCB,
	AXA = 0x9F,
	ATX = 0xAB,
}

impl Cpu {
	pub fn aac(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		let data = self.read(addr);
		self.register_a &= data;
		self.set_zero_neg_flags(self.register_a);
		self.status.set_carry(self.status.get_negative());
	}

	pub fn aax(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		self.write(addr, self.register_a & self.register_x);
	}

	pub fn add_a_carry(&mut self, data: u8) {
		let sum = self.register_a as u16 + data as u16 + self.status.get_carry() as u16;

		let carry = sum > 0xff;
		self.status.set_carry(carry);

		let result = sum as u8;
		let overflow = (data ^ result) & (result ^ self.register_a) & 0x80 != 0;
		self.status.set_overflow(overflow);

		self.register_a = result;
		self.set_zero_neg_flags(self.register_a);
	}

	// ADC - Add with Carry
	// A,Z,C,N = A+M+C
	// This instruction adds the contents of a memory location to the accumulator together with the carry bit. If overflow occurs the carry bit is set, this enables multiple byte addition to be performed.
	pub fn adc(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		let data = self.read(addr);
		self.add_a_carry(data);
	}

	// AND - Logical AND
	// A,Z,N = A&M
	// A logical AND is performed, bit by bit, on the accumulator contents using the contents of a byte of memory.
	pub fn and(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		let data = self.read(addr);
		self.register_a &= data;
		self.set_zero_neg_flags(self.register_a);
	}

	// ARR - AND byte with accumulator
	// AND byte with accumulator, then rotate one bit right in accumulator and check bit 5 and 6.
	pub fn arr(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		let data = self.read(addr);

		self.register_a &= data;
		self.set_zero_neg_flags(self.register_a);

		self.ror_a(mode);

		let result = self.register_a;
		let b5 = (result >> 5) & 1;
		let b6 = (result >> 6) & 1;
		self.status.set_carry(b5 == 1);
		self.status.set_overflow(b5 ^ b6 == 1);
		self.set_zero_neg_flags(result);
	}

	// ASL - Arithmetic Shift Left
	// A,Z,C,N = M*2 or M,Z,C,N = M*2
	// This operation shifts all the bits of the accumulator or memory contents one bit left. Bit 0 is set to 0 and bit 7 is placed in the carry flag. The effect of this operation is to multiply the memory contents by 2 (ignoring 2's complement considerations), setting the carry if the result will not fit in 8 bits.
	pub fn asl(&mut self, _mode: &AddressingMode) {
		self.asl_ext();
	}

	pub fn asl_ext(&mut self) -> u8 {
		let mut data = self.register_a;
		self.status.set_carry(data >> 7 == 1);
		data <<= 1;
		self.register_a = data;
		self.set_zero_neg_flags(self.register_a);
		data
	}

	// ASR
	// AND byte with accumulator, then shift right one bit in accumu-lator.
	pub fn asr(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		let data = self.read(addr);
		self.register_a &= data;
		self.set_zero_neg_flags(self.register_a);
		self.lsr_a(mode);
	}

	// TXA - Transfer X to Accumulator
	// A = X
	// Copies the current contents of the X register into the accumulator and sets the zero and negative flags as appropriate.
	pub fn txa(&mut self, _mode: &AddressingMode) {
		self.register_a = self.register_x;
		self.set_zero_neg_flags(self.register_a);
	}

	// TXS - Transfer X to Stack Pointer
	// S = X
	// Copies the current contents of the X register into the stack register.
	pub fn txs(&mut self, _mode: &AddressingMode) {
		self.stack_pointer = self.register_x;
	}

	// TYA - Transfer Y to Accumulator
	// A = Y
	// Copies the current contents of the Y register into the accumulator and sets the zero and negative flags as appropriate.
	pub fn tya(&mut self, _mode: &AddressingMode) {
		self.register_a = self.register_y;
		self.set_zero_neg_flags(self.register_a);
	}

	pub fn xas(&mut self, _mode: &AddressingMode) {
		let data = self.register_a & self.register_x;
		self.stack_pointer = data;
		let addr = self.read_u16(self.program_counter) + self.register_y as u16;

		let data = ((addr >> 8) as u8 + 1) & self.stack_pointer;
		self.write(addr, data);
	}

	// XAA - ???
	// This opcode results in UB. Its effects were replicated below;
	// It's works by writing X register to A, setting zero negative flags on register a, and anding it with a single argument.
	pub fn xaa(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		let data = self.read(addr);
		self.register_a = self.register_x;
		self.set_zero_neg_flags(self.register_a);
		self.register_a &= data;
	}

	// TSX - Transfer Stack Pointer to X
	// X = S
	// Copies the current contents of the stack register into the X register and sets the zero and negative flags as appropriate.
	pub fn tsx(&mut self, _mode: &AddressingMode) {
		self.register_x = self.stack_pointer;
		self.set_zero_neg_flags(self.register_x);
	}

	// TAY - Transfer Accumulator to Y
	// Y = A
	// Copies the current contents of the accumulator into the Y register and sets the zero and negative flags as appropriate.
	pub fn tay(&mut self, _mode: &AddressingMode) {
		self.register_y = self.register_a;
		self.set_zero_neg_flags(self.register_y);
	}

	// TAX - Transfer Accumulator to X
	// X = A
	// Copies the current contents of the accumulator into the X register and sets the zero and negative flags as appropriate.
	pub fn tax(&mut self, _mode: &AddressingMode) {
		self.register_x = self.register_a;
		self.set_zero_neg_flags(self.register_x);
	}

	// SYA
	// AND Y register with the high byte of the target address of the argument
	pub fn sya(&mut self, _mode: &AddressingMode) {
		let addr = self.read_u16(self.program_counter) + self.register_x as u16;
		let data = self.register_y & ((addr >> 8) as u8 + 1);
		self.write(addr, data);
	}

	// SXA
	// AND X register with the high byte of the target address of the argument
	pub fn sxa(&mut self, _mode: &AddressingMode) {
		let addr = self.read_u16(self.program_counter) + self.register_y as u16;
		let data = self.register_x & ((addr >> 8) as u8 + 1);
		self.write(addr, data);
	}

	// STY - Store Y Register
	// M = Y
	// Stores the contents of the Y register into memory.
	pub fn sty(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		self.write(addr, self.register_y);
	}

	// STX - Store X Register
	// M = X
	// Stores the contents of the X register into memory.
	pub fn stx(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		self.write(addr, self.register_x);
	}

	// STA - Store Accumulator
	// M = A
	// Stores the contents of the accumulator into memory.
	pub fn sta(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		self.write(addr, self.register_a);
	}

	// SRE
	// Shift right one bit in memory, then EOR accumulator with memory.
	pub fn sre(&mut self, mode: &AddressingMode) {
		let data = self.lsr_m_ext(mode);
		self.register_a ^= data;
		self.set_zero_neg_flags(self.register_a);
	}

	// SLO
	// Shift left one bit in memory, then OR accumulator with memory.
	pub fn slo(&mut self, _mode: &AddressingMode) {
		let data = self.asl_ext();
		self.register_a |= data;
		self.set_zero_neg_flags(self.register_a);
	}

	// SEI - Set Interrupt Disable
	// I = 1
	// Set the interrupt disable flag to one.
	pub fn sei(&mut self, _mode: &AddressingMode) {
		self.status.set_interrupt(true);
	}

	// SED - Set Decimal Flag
	// D = 1
	// Set the decimal mode flag to one.
	pub fn sed(&mut self, _mode: &AddressingMode) {
		self.status.set_decimal(true);
	}

	// SEC - Set Carry Flag
	// C = 1
	// Set the carry flag to one.
	pub fn sec(&mut self, _mode: &AddressingMode) {
		self.status.set_carry(true);
	}

	// SBC - Subtract with Carry
	// A,Z,C,N = A-M-(1-C)
	// This instruction subtracts the contents of a memory location to the accumulator together with the not of the carry bit. If overflow occurs the carry bit is clear, this enables multiple byte subtraction to be performed.
	pub fn sbc(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		let data = self.read(addr);
		let data = ((data as i8).wrapping_neg().wrapping_sub(1)) as u8;
		let sum = self.register_a as u16 + data as u16 + self.status.get_carry() as u16;

		let carry = sum > 0xff;
		self.status.set_carry(carry);

		let result = sum as u8;
		let overflow = (data ^ result) & (result ^ self.register_a) & 0x80 != 0;
		self.status.set_overflow(overflow);

		self.register_a = result;
		self.set_zero_neg_flags(self.register_a);
	}

	// RTS - Return from Subroutine
	// The RTS instruction is used at the end of a subroutine to return to the calling routine. It pulls the program counter (minus one) from the stack.
	pub fn rts(&mut self, _mode: &AddressingMode) {
		self.program_counter = self.pop_u16() + 1;
	}

	// RTI - Return from Interrupt
	// The RTI instruction is used at the end of an interrupt processing routine. It pulls the processor flags from the stack followed by the program counter.
	pub fn rti(&mut self, _mode: &AddressingMode) {
		*self.status = self.pop();
		self.status.set_break_min(false);
		self.status.set_break_max(true);
		self.program_counter = self.pop_u16();
	}

	// RRA
	// Rotate one bit right in memory, then add memory to accumulator (with carry).
	pub fn rra(&mut self, mode: &AddressingMode) {
		let data = self.ror_m_ext(mode);
		self.add_a_carry(data);
	}

	pub fn ror_m_ext(&mut self, mode: &AddressingMode) -> u8 {
		let addr = self.get_operand_address(mode);
		let mut data = self.read(addr);
		let carry = self.status.get_carry();
		self.status.set_carry(data & 1 == 1);
		data >>= 1;
		if carry {
			data |= 0b1000_0000;
		}
		self.write(addr, data);
		self.set_zero_neg_flags(data);
		data
	}

	// ROR - Rotate Right
	// Move each of the bits in either A or M one place to the right. Bit 7 is filled with the current value of the carry flag whilst the old bit 0 becomes the new carry flag value.
	pub fn ror_a(&mut self, _mode: &AddressingMode) {
		let mut data = self.register_a;
		let carry = self.status.get_carry();
		self.status.set_carry(data & 1 == 1);
		data >>= 1;
		if carry {
			data |= 0b1000_0000;
		}
		self.register_a = data;
		self.set_zero_neg_flags(self.register_a);
	}

	// ROR - Rotate Right
	// Move each of the bits in either A or M one place to the right. Bit 7 is filled with the current value of the carry flag whilst the old bit 0 becomes the new carry flag value.
	pub fn ror_m(&mut self, mode: &AddressingMode) {
		self.ror_m_ext(mode);
	}

	pub fn rol_m_ext(&mut self, mode: &AddressingMode) -> u8 {
		let addr = self.get_operand_address(mode);
		let mut data = self.read(addr);
		let carry = self.status.get_carry();
		self.status.set_carry(data >> 7 == 1);
		data <<= 1;
		if carry {
			data |= 1;
		}
		self.write(addr, data);
		self.set_zero_neg_flags(data);
		data
	}

	// ROL - Rotate Left
	// Move each of the bits in either A or M one place to the left. Bit 0 is filled with the current value of the carry flag whilst the old bit 7 becomes the new carry flag value.
	pub fn rol_a(&mut self, _mode: &AddressingMode) {
		let mut data = self.register_a;
		let carry = self.status.get_carry();
		self.status.set_carry(data >> 7 == 1);
		data <<= 1;
		if carry {
			data |= 1;
		}
		self.register_a = data;
		self.set_zero_neg_flags(self.register_a);
	}

	// ROL - Rotate Left
	// Move each of the bits in either A or M one place to the left. Bit 0 is filled with the current value of the carry flag whilst the old bit 7 becomes the new carry flag value.
	pub fn rol_m(&mut self, mode: &AddressingMode) {
		self.rol_m_ext(mode);
	}

	// RLA
	// Rotate one bit left in memory, then AND accumulator with memory.
	pub fn rla(&mut self, mode: &AddressingMode) {
		let data = self.rol_m_ext(mode);
		self.register_a &= data;
		self.set_zero_neg_flags(self.register_a);
	}

	// PLP - Pull Processor Status
	// Pulls an 8 bit value from the stack and into the processor flags. The flags will take on new states as determined by the value pulled.
	pub fn plp(&mut self, _mode: &AddressingMode) {
		*self.status = self.pop();
		self.status.set_break(true);
	}

	// PLA - Pull Accumulator
	// Pulls an 8 bit value from the stack and into the accumulator. The zero and negative flags are set as appropriate.
	pub fn pla(&mut self, _mode: &AddressingMode) {
		self.register_a = self.pop();
		self.set_zero_neg_flags(self.register_a);
	}

	// PHP - Push Processor Status
	// Pushes a copy of the status flags on to the stack.
	pub fn php(&mut self, _mode: &AddressingMode) {
		let mut flags = *self.status;
		flags |= 0b0011_0000;
		self.push(flags);
	}

	// PHA - Push Accumulator
	// Pushes a copy of the accumulator on to the stack.
	pub fn pha(&mut self, _mode: &AddressingMode) {
		self.write((crate::cpu::STACK as u16) + self.stack_pointer as u16, self.register_a);
		self.stack_pointer = self.stack_pointer.wrapping_sub(1);
	}

	// ORA - Logical Inclusive OR
	// A,Z,N = A|M
	// An inclusive OR is performed, bit by bit, on the accumulator contents using the contents of a byte of memory.
	pub fn ora(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		let data = self.read(addr);
		self.register_a |= data;
		self.set_zero_neg_flags(self.register_a);
	}

	// NOP - No operation
	// Doesn't perform any operation
	pub fn nop(&mut self, _mode: &AddressingMode) {}

	// LSR - Logical Shift Right
	// A,C,Z,N = A/2 or M,C,Z,N = M/2
	// Each of the bits in A or M is shift one place to the right. The bit that was in bit 0 is shifted into the carry flag. Bit 7 is set to zero.
	pub fn lsr_a(&mut self, _mode: &AddressingMode) {
		let mut data = self.register_a;
		self.status.set_carry(data & 1 == 1);
		data >>= 1;
		self.register_a = data;
		self.set_zero_neg_flags(self.register_a);
	}

	// LSR - Logical Shift Right
	// A,C,Z,N = A/2 or M,C,Z,N = M/2
	// Each of the bits in A or M is shift one place to the right. The bit that was in bit 0 is shifted into the carry flag. Bit 7 is set to zero.
	pub fn lsr_m(&mut self, mode: &AddressingMode) {
		self.lsr_m_ext(mode);
	}

	pub fn lsr_m_ext(&mut self, mode: &AddressingMode) -> u8 {
		let addr = self.get_operand_address(mode);
		let mut data = self.read(addr);
		self.status.set_carry(data & 1 == 1);
		data >>= 1;
		self.write(addr, data);
		self.set_zero_neg_flags(data);
		data
	}

	// LDY - Load Y Register
	// Y,Z,N = M
	// Loads a byte of memory into the Y register setting the zero and negative flags as appropriate.
	pub fn ldy(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		let value = self.read(addr);

		self.register_y = value;
		self.set_zero_neg_flags(self.register_y);
	}

	// LDX - Load X Register
	// X,Z,N = M
	// Loads a byte of memory into the X register setting the zero and negative flags as appropriate.
	pub fn ldx(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		let value = self.read(addr);

		self.register_x = value;
		self.set_zero_neg_flags(self.register_x);
	}

	// LDA - Load Accumulator
	// A,Z,N = M
	// Loads a byte of memory into the accumulator setting the zero and negative flags as appropriate.
	pub fn lda(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		let value = self.read(addr);

		self.register_a = value;
		self.set_zero_neg_flags(self.register_a);
	}

	pub fn lax(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		let data = self.read(addr);
		self.register_x = data;
		self.register_a = self.register_x;
		self.set_zero_neg_flags(self.register_a);
	}

	// LAR
	// AND memory with stack pointer, transfer result to accumulator, X register and stack pointer.
	pub fn lar(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		let data = self.read(addr) & self.stack_pointer;
		self.register_a = data;
		self.register_x = data;
		self.stack_pointer = data;
		self.set_zero_neg_flags(data);
	}

	// JSR - Jump to Subroutine
	// The JSR instruction pushes the address (minus one) of the return point on to the stack and then sets the program counter to the target memory address.
	pub fn jsr(&mut self, _mode: &AddressingMode) {
		self.push_u16(self.program_counter + 2 - 1);
		self.program_counter = self.read_u16(self.program_counter);
	}

	pub fn jmp_absolute(cpu: &mut Cpu, _mode: &AddressingMode) {
		cpu.program_counter = cpu.read_u16(cpu.program_counter);
	}

	// JMP - Jump
	// Sets the program counter to the address specified by the operand..
	// Note: This instruction was buggy on the original hardware. This implementation accounts for that.
	pub fn jmp(cpu: &mut Cpu, _mode: &AddressingMode) {
		cpu.program_counter = {
			let addr = cpu.read_u16(cpu.program_counter);
			if addr & 0x00FF == 0x00FF {
				let lo = cpu.read(addr);
				let hi = cpu.read(addr & 0xFF00);
				(hi as u16) << 8 | (lo as u16)
			} else {
				cpu.read_u16(addr)
			}
		}
	}

	// ISC - INC subtract
	// Increase memory by one, then subtract memory from accu-mulator (with borrow)
	pub fn isc(&mut self, mode: &AddressingMode) {
		let data = (self.inc_ret(mode) as i8).wrapping_neg().wrapping_sub(1) as u8;
		let sum = self.register_a as u16 + data as u16 + self.status.get_carry() as u16;
		let carry = sum > 0xff;

		self.status.set_carry(carry);
		let result = sum as u8;

		self.status.set_overflow((data ^ result) & (result ^ self.register_a) & 0x80 != 0);

		self.register_a = result;
		self.set_zero_neg_flags(self.register_a);
	}

	// INY - Increment Y Register
	// Y,Z,N = Y+1
	// Adds one to the Y register setting the zero and negative flags as appropriate.
	pub fn iny(&mut self, _mode: &AddressingMode) {
		self.register_y = self.register_y.overflowing_add(1).0;
		self.set_zero_neg_flags(self.register_y);
	}

	// INX - Increment X Register
	// X,Z,N = X + 1
	// Adds one to the X register setting the zero and negative flags as appropriate.
	pub fn inx(&mut self, _mode: &AddressingMode) {
		self.register_x = self.register_x.overflowing_add(1).0;
		self.set_zero_neg_flags(self.register_x);
	}

	// INC - Increment Memory
	// M,Z,N = M+1
	// Adds one to the value held at a specified memory location setting the zero and negative flags as appropriate.
	pub fn inc(&mut self, mode: &AddressingMode) {
		self.inc_ret(mode);
	}

	// INC - Increment Memory
	// M,Z,N = M+1
	// Adds one to the value held at a specified memory location setting the zero and negative flags as appropriate.
	pub fn inc_ret(&mut self, mode: &AddressingMode) -> u8 {
		let addr = self.get_operand_address(mode);
		let mut data = self.read(addr);
		data = data.wrapping_add(1);
		self.write(addr, data);
		self.set_zero_neg_flags(data);
		data
	}

	// EOR - Exclusive OR
	// A,Z,N = A^M
	// An exclusive OR is performed, bit by bit, on the accumulator contents using the contents of a byte of memory.
	pub fn eor(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		let data = self.read(addr);
		self.register_a ^= data;
		self.set_zero_neg_flags(self.register_a);
	}

	// DEY - Decrement Y Register
	// Y,Z,N = Y-1
	// Subtracts one from the Y register setting the zero and negative flags as appropriate.
	pub fn dey(&mut self, _mode: &AddressingMode) {
		self.register_y = self.register_y.overflowing_sub(1).0;
		self.set_zero_neg_flags(self.register_y);
	}

	// DEX - Decrement X Register
	// X,Z,N = X-1
	// Subtracts one from the X register setting the zero and negative flags as appropriate.
	pub fn dex(&mut self, _mode: &AddressingMode) {
		self.register_x = self.register_x.overflowing_sub(1).0;
		self.set_zero_neg_flags(self.register_x);
	}

	// DEC - Decrement Memory
	// M,Z,N = M-1
	// Subtracts one from the value held at a specified memory location setting the zero and negative flags as appropriate.
	pub fn dec(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		let mut data = self.read(addr);
		data = data.wrapping_sub(1);
		self.write(addr, data);
		self.set_zero_neg_flags(data);
	}

	// DCP
	// Subtract one from memory.
	pub fn dcp(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		let mut data = self.read(addr);

		data = data.wrapping_sub(1);
		self.write(addr, data);

		if data <= self.register_a {
			self.status.set_carry(true);
		}

		self.set_zero_neg_flags(self.register_a.wrapping_sub(data));
	}

	// CPY - Compare Y Register
	// Z,C,N = Y-M
	// This instruction compares the contents of the Y register with another memory held value and sets the zero and carry flags as appropriate.
	pub fn cpy(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		let data = self.read(addr);
		self.status.set_carry(self.register_y >= data);
		self.status.set_zero(self.register_y == data);
		self.set_zero_neg_flags(self.register_y.wrapping_sub(data));
	}

	// CPX - Compare X Register
	// Z,C,N = X-M
	// This instruction compares the contents of the X register with another memory held value and sets the zero and carry flags as appropriate.
	pub fn cpx(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		let data = self.read(addr);
		self.status.set_carry(self.register_x >= data);
		self.status.set_zero(self.register_x == data);
		self.set_zero_neg_flags(self.register_x.wrapping_sub(data));
	}

	// CMP - Compare
	// Z,C,N = A-M
	// This instruction compares the contents of the accumulator with another memory held value and sets the zero and carry flags as appropriate.
	pub fn cmp(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		let data = self.read(addr);
		self.status.set_carry(self.register_a >= data);
		self.status.set_zero(self.register_a == data);
		self.set_zero_neg_flags(self.register_a.wrapping_sub(data));
	}

	// CLV - Clear Overflow Flag
	// V = 0
	// Clears the overflow flag.
	pub fn clv(&mut self, _mode: &AddressingMode) {
		self.status.set_overflow(false);
	}

	// CLI - Clear Interrupt Disable
	// I = 0
	// Clears the interrupt disable flag allowing normal interrupt requests to be serviced.
	pub fn cli(&mut self, _mode: &AddressingMode) {
		self.status.set_interrupt(false);
	}

	// This is unused on NES, but I have implemented it anyway because it's in the spec.
	// CLD - Clear Decimal Mode
	// D = 0
	pub fn cld(&mut self, _mode: &AddressingMode) {
		self.status.set_decimal(false);
	}

	// CLC - Clear Carry Flag
	// C = 0
	pub fn clc(&mut self, _mode: &AddressingMode) {
		self.status.set_carry(false);
	}

	// BVS - Branch if Overflow Set
	// If the overflow flag is set then add the relative displacement to the program counter to cause a branch to a new location.
	pub fn bvs(&mut self, _mode: &AddressingMode) {
		self.branch_if(self.status.get_overflow());
	}

	// BVC - Branch if Overflow Clear
	// If the overflow flag is clear then add the relative displacement to the program counter to cause a branch to a new location.
	pub fn bvc(&mut self, _mode: &AddressingMode) {
		self.branch_if(!self.status.get_overflow());
	}

	// BPL - Branch if Positive
	// If the negative flag is clear then add the relative displacement to the program counter to cause a branch to a new location.
	pub fn bpl(&mut self, _mode: &AddressingMode) {
		self.branch_if(!self.status.get_negative());
	}

	// BNE - BNE - Branch if Not Equal
	// If the zero flag is clear then add the relative displacement to the program counter to cause a branch to a new location.
	pub fn bne(&mut self, _mode: &AddressingMode) {
		self.branch_if(!self.status.get_zero());
	}

	// BMI - Branch if Minus
	// If the negative flag is set then add the relative displacement to the program counter to cause a branch to a new location.
	pub fn bmi(&mut self, _mode: &AddressingMode) {
		self.branch_if(self.status.get_negative());
	}

	// BIT - Bit Test
	// A & M, N = M7, V = M6
	// This instruction is used to test if one or more bits are set in a target memory location. The mask pattern in A is ANDed with the value in memory to set or clear the zero flag, but the result is not kept. Bits 7 and 6 of the value from memory are copied into the N and V flags.
	pub fn bit(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		let data = self.read(addr);
		self.status.set_zero(self.register_a & data == 0);
		self.status.set_negative(data & 0b1000_0000 > 0);
		self.status.set_overflow(data & 0b0100_0000 > 0);
	}

	// BEQ - Branch if Equal
	// If the zero flag is set then add the relative displacement to the program counter to cause a branch to a new location.
	pub fn beq(&mut self, _mode: &AddressingMode) {
		self.branch_if(self.status.get_zero());
	}

	// BCS - Branch if Carry Set
	// If the carry flag is set then add the relative displacement to the program counter to cause a branch to a new location.
	pub fn bcs(&mut self, _mode: &AddressingMode) {
		self.branch_if(self.status.get_carry());
	}


	// BCC - Branch if Carry Clear
	// If the carry flag is clear then add the relative displacement to the program counter to cause a branch to a new location.
	pub fn bcc(&mut self, _mode: &AddressingMode) {
		self.branch_if(!self.status.get_carry());
	}

	// AXS - AND X register with accumulator
	// Perform a logical AND on X register with accumulator and store result in X register, then subtract byte from X register (without borrow).
	pub fn axs(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		let data = self.read(addr);
		let xa = self.register_x & self.register_a;
		let result = xa.wrapping_sub(data);

		if data <= xa {
			self.status.set_carry(true);
		}

		self.register_x = result;
		self.set_zero_neg_flags(self.register_x);
	}

	// AXA - AND X register with accumulator
	// Perform a logical AND on x register with accumulator, then AND result with 7 and store in memory.
	pub fn axa_in(&mut self, _mode: &AddressingMode) {
		let pos: u8 = self.read(self.program_counter);
		let addr = self.read_u16(pos as u16) + self.register_y as u16;
		let data = self.register_a & self.register_x & 0b0000_0111;
		self.write(addr, data);
	}

	// AXA - AND X register with accumulator
	// Perform a logical AND on x register with accumulator, then AND result with 7 and store in memory.
	pub fn axa_ab(&mut self, _mode: &AddressingMode) {
		let addr = self.read_u16(self.program_counter) + self.register_y as u16;
		let data = self.register_a & self.register_x & 0b0000_0111;
		self.write(addr, data);
	}

	// AND byte with accumulator
	// Perform a logical AND on byte with accumulator, then transfer accumulator to X register.
	pub fn atx(&mut self, mode: &AddressingMode) {
		let addr = self.get_operand_address(mode);
		let data = self.read(addr);

		self.register_a &= data;
		self.set_zero_neg_flags(self.register_a);

		self.tax(mode);
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use OpCode::*;

	#[test]
	fn test_aac() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9,0b1000_0001,AAC,0b1000_0100,0x00]);
		assert_eq!(cpu.register_a, 0b1000_0000);
		assert!(cpu.status.get_negative())
	}

	#[test]
	fn test_aax() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0b1000_0001, 0xa2, 0b1000_0100, AAX, 0x00]);
		assert!(cpu.bus.vram_contains(0b1000_0000))
	}

	#[test]
	fn test_adc_no_carry() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 1, ADC, 3, 0x00]);
		assert_eq!(cpu.register_a, 4);
	}

	#[test]
	fn test_adc_carry() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0xff, ADC, 3, 0x00]);
		assert_eq!(cpu.register_a, 2);
		assert!(cpu.status.get_carry());
	}

	#[test]
	fn test_and() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0x01, AND, 0x03, 0x00]);
		assert_eq!(cpu.register_a, 0x01);
	}

	#[test]
	fn test_asl() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 2, ASL, 0x00]);
		assert_eq!(cpu.register_a, 4)
	}

	#[test]
	fn test_asl_overflow() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xA9, 0b1000_0001, ASL, 0x00]);
		assert_eq!(cpu.register_a, 2);
		assert_eq!(*cpu.status & 0b0000_0001, 1);
	}

	#[test]
	fn test_asr() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0b1000_0011, ASR, 0b1000_0001, 0x00]);
		assert_eq!(cpu.register_a, 0b0100_0000)
	}

	#[test]
	fn test_txa_move_x_to_a() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa2, 69, TXA, 0x00]);
		assert_eq!(cpu.register_a, 69)
	}

	#[test]
	fn test_tya_move_y_to_a() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa0, 69, TYA, 0x00]);
		assert_eq!(cpu.register_a, 69)
	}

	#[test]
	fn test_tay_move_a_to_y() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xA9, 69, TAY, 0x00]);
		assert_eq!(cpu.register_y, 69)
	}

	#[test]
	fn test_tax_move_a_to_x() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 69, TAX, 0x00]);
		assert_eq!(cpu.register_x, 69)
	}

	#[test]
	fn test_sty_store_data() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa0, 0x43, STY, 0x00]);
		assert!(cpu.bus.vram_contains(0x43));
	}

	#[test]
	fn test_stx_store_data() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa2, 0x43, STX, 0x00]);
		assert!(cpu.bus.vram_contains(0x43));
	}

	#[test]
	fn test_sta_store_data() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0x43, STA, 0x00]);
		assert!(cpu.bus.vram_contains(0x43));
	}

	#[test]
	fn test_sei() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![SEI, 0x00]);
		assert!(cpu.status.get_interrupt());
	}

	#[test]
	fn test_sed() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![SED, 0x00]);
		assert!(cpu.status.get_decimal());
	}

	#[test]
	fn test_sec() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![SEC, 0x00]);
		assert!(cpu.status.get_carry());
	}

	#[test]
	fn test_sbc_overflow() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 1, SBC, 3, 0x00]);
		assert_eq!(cpu.register_a, 253);
		assert!(!cpu.status.get_carry());
	}

	#[test]
	fn test_sbc_no_overflow() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0xff, SBC, 3, 0x00]);
		assert_eq!(cpu.register_a, 251);
		assert!(cpu.status.get_carry());
	}

	#[test]
	fn test_rts_empty() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![RTS, 0x00]);
		assert_eq!(cpu.stack_pointer, 255);
	}

	#[test]
	pub fn test_ror() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0b1000_0001, ROR, 0x00]);
		assert_eq!(cpu.register_a, 64);
		assert!(cpu.status.get_carry())
	}

	#[test]
	pub fn test_rol() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0b1000_0001, ROL, 0x00]);
		assert_eq!(cpu.register_a, 2);
		assert!(cpu.status.get_carry())
	}

	#[test]
	fn test_php() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 16, PHP, 0x00]);
		assert!(cpu.bus.vram_contains(*cpu.status | 0b0011_0000))
	}

	#[test]
	fn test_pha() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 16, PHA, 0x00]);
		assert!(cpu.bus.vram_contains(16));
	}

	#[test]
	fn test_ora() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0x08, ORA, 0x07, 0x00]);
		assert_eq!(cpu.register_a, 15);
	}

	#[test]
	fn test_lsr_a() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 5, LSR, 0x00]);
		assert_eq!(cpu.register_a, 2);
		assert!(cpu.status.get_carry())
	}

	#[test]
	fn test_ldy_load_data() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![LDY, 0x43, 0x00]);
		assert_eq!(cpu.register_y, 0x43);
		assert!(*cpu.status & 0b0000_0010 == 0b00);
		assert!(*cpu.status & 0b1000_0000 == 0);
	}

	#[test]
	fn test_ldy_zero_flag() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![LDY, 0x00, 0x00]);
		assert!(*cpu.status & 0b0000_0010 == 0b10);
	}

	#[test]
	fn test_ldx_load_data() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![LDX, 0x43, 0x00]);
		assert_eq!(cpu.register_x, 0x43);
		assert!(*cpu.status & 0b0000_0010 == 0b00);
		assert!(*cpu.status & 0b1000_0000 == 0);
	}

	#[test]
	fn test_ldx_zero_flag() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![LDX, 0x00, 0x00]);
		assert!(*cpu.status & 0b0000_0010 == 0b10);
	}

	#[test]
	fn test_lda_load_data() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![LDA, 0x43, 0x00]);
		assert_eq!(cpu.register_a, 0x43);
		assert!(*cpu.status & 0b0000_0010 == 0b00);
		assert!(*cpu.status & 0b1000_0000 == 0);
	}

	#[test]
	fn test_lda_zero_flag() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![LDA, 0x00, 0x00]);
		assert!(*cpu.status & 0b0000_0010 == 0b10);
	}

	#[test]
	fn test_lax() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xA7, 0x00]);
		assert_eq!(cpu.register_a, cpu.register_x);
	}

	#[test]
	fn test_jmp() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0x6C, 10, 0x00]);
		assert_eq!(cpu.program_counter, 2);

		cpu.interpret(vec![0x4C, 10, 0x00]);
		assert_eq!(cpu.program_counter, 12);
	}

	#[test]
	fn test_iny_increment() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![INY, 0x00]);
		assert_eq!(cpu.register_y, 1)
	}

	#[test]
	fn test_iny_overflow() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa0, 0xff, INY, 0x00]);
		assert_eq!(cpu.register_y, 0)
	}

	#[test]
	fn test_inx_increment() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![INX, 0x00]);
		assert_eq!(cpu.register_x, 1)
	}

	#[test]
	fn test_inx_overflow() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0xff, 0xaa, INX, INX, 0x00]);
		assert_eq!(cpu.register_x, 1)
	}

	#[test]
	fn test_eor() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0b0000_0001, EOR, 0b0000_0011, 0x00]);
		assert_eq!(cpu.register_a, 0b0000_0010);
	}

	#[test]
	fn test_dey_decrement() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xA0, 10, DEY, 0x00]);
		assert_eq!(cpu.register_y, 9)
	}

	#[test]
	fn test_dex_decrement() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 10, 0xaa, DEX, 0x00]);
		assert_eq!(cpu.register_x, 9)
	}

	#[test]
	fn test_cpy_less() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xA0, 5, CPY, 3, 0x00]);
		assert!(cpu.status.get_carry());
		assert!(!cpu.status.get_zero());
	}

	#[test]
	fn test_cpy_equal() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xA0, 5, CPY, 5, 0x00]);
		assert!(cpu.status.get_carry());
		assert!(cpu.status.get_zero());
	}

	#[test]
	fn test_cpx_less() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa2, 5, CPX, 3, 0x00]);
		assert!(cpu.status.get_carry());
		assert!(!cpu.status.get_zero());
	}

	#[test]
	fn test_cpx_equal() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa2, 5, CPX, 5, 0x00]);
		assert!(cpu.status.get_carry());
		assert!(cpu.status.get_zero());
	}

	#[test]
	fn test_cmp_less() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 5, CMP, 3, 0x00]);
		assert!(cpu.status.get_carry());
		assert!(!cpu.status.get_zero());
	}

	#[test]
	fn test_cmp_equal() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 5, CMP, 5, 0x00]);
		assert!(cpu.status.get_carry());
		assert!(cpu.status.get_zero());
	}

	#[test]
	fn test_overflow_clear() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![CLV, 0x00]);
		assert!(!cpu.status.get_overflow())
	}

	#[test]
	fn test_interrupt_clear() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![CLI, 0x00]);
		assert!(!cpu.status.get_interrupt())
	}

	#[test]
	fn test_decimal_clear() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![CLD, 0x00]);
		assert!(!cpu.status.get_decimal())
	}

	#[test]
	fn test_carry_clear() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0xff, 0x0a, CLC, 0x00]);
		assert!(!cpu.status.get_carry())
	}

	#[test]
	fn test_bvs_clear() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![BVS, 0x00, 0x00, 0xa9, 4, 0x00]);
		assert_eq!(cpu.register_a, 0)
	}

	#[test]
	fn test_bvs_set() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0x50, 0x69, 0x50, BVS, 0x00, 0x00, 0xa9, 4, 0x00]);
		assert_eq!(cpu.register_a, 4)
	}

	#[test]
	fn test_bvc_clear() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![BVC, 0x00, 0x00, 0xa9, 4, 0x00]);
		assert_eq!(cpu.register_a, 4)
	}

	#[test]
	fn test_bvc_set() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0x50, 0x69, 0x50, BVC, 0x00, 0x00, 0xa9, 4, 0x00]);
		assert_eq!(cpu.register_a, 0x50 + 0x50)
	}

	#[test]
	fn test_bpl_negative() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0b1000_0000, BPL, 0x00, 0x00, 0xa9, 4, 0x00]);
		assert_eq!(cpu.register_a, 0b1000_0000)
	}

	#[test]
	fn test_bpl_positive() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0b0000_0000, BPL, 0x00, 0x00, 0xa9, 4, 0x00]);
		assert_eq!(cpu.register_a, 4)
	}

	#[test]
	fn test_bne_zero() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0, BNE, 0x00, 0x00, 0xa9, 4, 0x00]);
		assert_eq!(cpu.register_a, 0)
	}

	#[test]
	fn test_bne_not_zero() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 1, BNE, 0x00, 0x00, 0xa9, 4, 0x00]);
		assert_eq!(cpu.register_a, 4)
	}

	#[test]
	fn test_bmi_negative() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0b1000_0000, BMI, 0x00, 0x00, 0xa9, 4, 0x00]);
		assert_eq!(cpu.register_a, 4)
	}

	#[test]
	fn test_bmi_positive() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0b0000_0000, BMI, 0x00, 0x00, 0xa9, 4, 0x00]);
		assert_eq!(cpu.register_a, 0)
	}


	#[test]
	fn test_beq_zero() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0, BEQ, 0x00, 0x00, 0xa9, 4, 0x00]);
		assert_eq!(cpu.register_a, 4)
	}

	#[test]
	fn test_beq_not_zero() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 1, BEQ, 0x00, 0x00, 0xa9, 4, 0x00]);
		assert_eq!(cpu.register_a, 1)
	}

	#[test]
	fn test_bcs_clear() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![BCS, 0x00, 0x00, 0xa9, 4, 0x00]);
		assert_eq!(cpu.register_a, 0)
	}

	#[test]
	fn test_bcs_set() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0xff, 0x69, 1, BCS, 0x00, 0x00, 0xa9, 4, 0x00]);
		assert_eq!(cpu.register_a, 4)
	}

	#[test]
	fn test_bcc_clear() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![BCC, 0x00, 0x00, 0xa9, 4, 0x00]);
		assert_eq!(cpu.register_a, 4)
	}

	#[test]
	fn test_bcc_set() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0xff, 0x69, 1, BCC, 0x00, 0x00, 0xa9, 4, 0x00]);
		assert_eq!(cpu.register_a, 0)
	}

	#[test]
	fn test_axs() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0b0000_1111, 0xa2, 0b0000_0111, AXS, 0b0000_0001, 0x00]);
		assert_eq!(cpu.register_x, 0b0000_0110)
	}

	#[test]
	fn test_axa_indirect() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0b1100_0110, 0xa2, 0b0110_0011, 0x93, 0x00]);
		assert!(cpu.bus.vram_contains(0b0000_0010));
	}

	#[test]
	fn test_axa_absolute() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0b1100_0110, 0xa2, 0b0110_0011, AXA, 0x00]);
		assert!(cpu.bus.vram_contains(0b0000_0010));
	}

	#[test]
	fn test_atx() {
		let mut cpu = Cpu::new();
		cpu.interpret(vec![0xa9, 0b1100_1111, ATX, 0b1001_0010, 0x00]);
		assert_eq!(cpu.register_a, cpu.register_x);
		assert_eq!(cpu.register_x, 0b1000_0010)
	}
}