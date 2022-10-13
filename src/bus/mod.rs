use super::memory::Memory;

const RAM_START: u16 = 0x0000;
const RAM_END: u16 = 0x1FFF;
const PPU_START: u16 = 0x2000;
const PPU_END: u16 = 0x3FFF;

const fn mirror_addr(addr: u16) -> u16 {
	addr & 0b0000_0111_1111_1111
}

pub struct Bus {
	vram: [u8; 2048]
}

impl Bus {
	pub fn new() -> Self{
		Bus {
			vram: [0; 2048]
		}
	}
}

#[cfg(test)]
impl Bus {
	pub fn vram_contains(&self, value: u8) -> bool {
		self.vram.contains(&value)
	}
}

impl Memory for Bus {
	fn read(&self, addr: u16) -> u8 {
		match addr {
			RAM_START ..= RAM_END => {
				self.vram[mirror_addr(addr) as usize]
			}
			PPU_START ..= PPU_END => {
				todo!("PPU")
			}
			_ => 0
		}
	}

	fn write(&mut self, addr: u16, data: u8) {
		match addr {
			RAM_START ..= RAM_END => {
				self.vram[mirror_addr(addr) as usize] = data;
			}
			PPU_START ..= PPU_END => {
				todo!("PPU");
			}
			_ => {}
		}
	}
}