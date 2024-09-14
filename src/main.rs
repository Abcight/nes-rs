use cpu::Cpu;
use cpu::*;

mod cpu;
mod bus;
mod memory;

fn main() {
	let mut cpu = Cpu::new();
	cpu.interpret(vec![
		LDA1.code,
		0x03,			// Load 0x03 into register A
		TAX.code, 		// Copy value from register A into register X
		DEX.code, 		// Decrease value at register X by one
		0x00,			// Break
	]);
	println!("{}", cpu.register_x); // Print the value stored in register X (2)
}