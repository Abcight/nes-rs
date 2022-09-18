use cpu::Cpu;

mod cpu;

fn main() {
	use cpu::ops::*;
	let mut cpu = Cpu::new();
	cpu.interpret(vec![
		lda::IMOP,
		0x03,      // Load 0x03 into register A
		tax::IMOP, // Copy value from register A into register X
		dex::IMOP, // Decrease value at register X by one
		0x00,      // Break
	]);
	println!("{}", cpu.register_x); // Print the value stored in register X (2)
}