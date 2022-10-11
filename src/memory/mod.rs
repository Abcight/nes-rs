pub trait Memory {
	fn read(&self, addr: u16) -> u8;
	fn write(&mut self, addr: u16, data: u8);
	fn read_u16(&self, pos: u16) -> u16;
	fn write_u16(&mut self, pos: u16, data: u16);
}