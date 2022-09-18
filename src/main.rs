use cpu::CPU;

mod cpu;

fn main() {
	let mut cpu = CPU::new();
	cpu.interpret(vec![
		cpu::ops::LDA,
		0x03,          // Load 0x03 into register A
		cpu::ops::TAX, // Copy value from register A into register B
		cpu::ops::DEX, // Decrease value at register B by one
		cpu::ops::BRK, // Break
	]);
	println!("{}", cpu.register_x); // Print the value stored in register B (2)
}
