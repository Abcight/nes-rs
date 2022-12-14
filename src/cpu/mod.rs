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