pub trait Memory {
	fn read(&self, addr: u16) -> u8;
	fn write(&mut self, addr: u16, data: u8);
	
	fn read_u16(&self, pos: u16) -> u16 {
		let lo = self.read(pos) as u16;
		let hi = self.read(pos + 1) as u16;
		(hi << 8) | (lo as u16)
	}

	fn write_u16(&mut self, pos: u16, data: u16) {
		let hi = (data >> 8) as u8;
		let lo = (data & 0xff) as u8;
		self.write(pos, lo);
		self.write(pos + 1, hi);
	}
}