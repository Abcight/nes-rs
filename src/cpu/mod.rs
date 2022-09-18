pub mod ops;

pub struct CPU {
	pub register_a: u8,
	pub register_x: u8,
	pub register_y: u8,
	pub status: u8,
	pub program_counter: u16,
}

impl CPU {
	pub fn new() -> Self {
		CPU {
			register_a: 0,
			register_x: 0,
			register_y: 0,
			status: 0,
			program_counter: 0,
		}
	}

	fn set_zero_neg_flags(&mut self, result: u8) {
		if result == 0 {
			self.status |= 0b0000_0010;
		} else {
			self.status &= 0b1111_1101;
		}

		if result & 0b1000_0000 != 0 {
			self.status |= 0b1000_0000;
		} else {
			self.status &= 0b0111_1111;
		}
	}

	pub fn interpret(&mut self, program: Vec<u8>) {
		self.program_counter = 0;
		loop {
			let opcode = program[self.program_counter as usize];
			self.program_counter += 1;
			match opcode {
				ops::LDA => {
					let param = program[self.program_counter as usize];
					self.program_counter += 1;
					self.lda(param);
				}
				ops::TAX => self.tax(),
				ops::INX => self.inx(),
				ops::DEX => self.dex(),
				0x00 => return,
				_ => todo!(),
			}
		}
	}
}
