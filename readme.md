# NES-RS
A simple Nintendo Entertaining System emulator written in pure rust.
**This project is a work-in-progess!!**

## Why?
I want to gain insight into the inner workings of the NES, and this project is a part of my research. In addition to that, I seek to learn more about the Rust programming language, and this project is a golden opportunity to achieve that goal.

## Progress
The project is undergoing a very minor refactor at the moment.
I have recognised things that could've been done in a much better way, and I think it's appropriate to address these issues, as this is first and foremost a learning exercise.

So far I have only began working on emulating the [Obelisk 6502 chip instructions](https://www.nesdev.org/obelisk-6502-guide/reference.html). Here you can see how many have already been implemented:
<table>
	<tr>
		<td>ADC🚧</td>
		<td>AND🚧</td>
		<td>ASL🚧</td>
		<td>BCC🚧</td>
		<td>BCS🚧</td>
		<td>BEQ🚧</td>
		<td>BIT🚧</td>
		<td>BMI🚧</td>
		<td>BNE🚧</td>
		<td>BPL🚧</td>
		<td>BRK🚧</td>
		<td>BVC🚧</td>
		<td>BVS🚧</td>
		<td>CLC🚧</td>
	</tr>
	<tr>
		<td>CLD🚧</td>
		<td>CLI🚧</td>
		<td>CLV🚧</td>
		<td>CMP🚧</td>
		<td>CPX🚧</td>
		<td>CPY🚧</td>
		<td>DEC🚧</td>
		<td>DEX🚧</td>
		<td>DEY🚧</td>
		<td>EOR🚧</td>
		<td>INC🚧</td>
		<td>INX🚧</td>
		<td>INY🚧</td>
		<td>JMP🚧</td>
	</tr>
	<tr>
		<td>JSR🚧</td>
		<td>LDA🚧</td>
		<td>LDX🚧</td>
		<td>LDY🚧</td>
		<td>LSR🚧</td>
		<td>NOP🚧</td>
		<td>ORA🚧</td>
		<td>PHA🚧</td>
		<td>PHP🚧</td>
		<td>PLA🚧</td>
		<td>PLP🚧</td>
		<td>ROL🚧</td>
		<td>ROR🚧</td>
		<td>RTI🚧</td>
	</tr>
	<tr>
		<td>RTS🚧</td>
		<td>SBC🚧</td>
		<td>SEC🚧</td>
		<td>SED🚧</td>
		<td>SEI🚧</td>
		<td>STA🚧</td>
		<td>STX🚧</td>
		<td>STY🚧</td>
		<td>TAX🚧</td>
		<td>TAY🚧</td>
		<td>TSX🚧</td>
		<td>TXA🚧</td>
		<td>TXS🚧</td>
		<td>TYA🚧</td>
	</tr>
</table>

Additionally, the chip shipped with certain unofficial opcodes that were left unused by design. However, some games still use these undocumented operations, so I am going to implement them for the sake of completeness. You can see the progress on that here:
<table>
	<tr>
		<td>AAC🚧</td>
		<td>AAX🚧</td>
		<td>ARR🚧</td>
		<td>ASR🚧</td>
		<td>ATX🚧</td>
		<td>AXA🚧</td>
		<td>AXS🚧</td>
		<td>DCP🚧</td>
		<td>DOP🚧</td>
		<td>ISC🚧</td>
		<td>KIL🚧</td>
		<td>LAR🚧</td>
		<td>LAX🚧</td>
		<td>NOP🚧</td>
	</tr>
	<tr>
		<td>RLA🚧</td>
		<td>RRA🚧</td>
		<td>SBC🚧</td>
		<td>SLO🚧</td>
		<td>SRE🚧</td>
		<td>SXA🚧</td>
		<td>SYA🚧</td>
		<td>TOP🚧</td>
		<td>XAA🚧</td>
		<td>XAS🚧</td>
	</tr>
</table>

<table>
	<tr>
		<th>Symbol</th>
		<th>Description</th>
	</tr>
	<tr>
		<td>🚧</td>
		<td>Implemented, not ran against NesTest</td>
	</tr>
	<tr>
		<td>✔️</td>
		<td>Implemented, passed NesTest</td>
	</tr>
	<tr>
		<td>❌</td>
		<td>Implemented, failed NesTest</td>
	</tr>
</table>

## Goals
My grandest goal is that of building a greater understanding of NES and its inner workings, as well as deepening my knowledge of Rust.
That wouldn't be possible without having a set path in mind, so I've decided that by the end of this learning experience I want to have an emulator capable of running the "Super Mario Bros".

## Sources
- [Call-A.P.P.L.E Obelisk 6502](https://www.callapple.org/obelisk-6502-registers/)
- [Obelisk.me guide (archived)](https://web.archive.org/web/20210909190432/http://www.obelisk.me.uk/6502/)
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [NesDev website](https://www.nesdev.org/)
- [NES emulator e-book](https://bugzmanov.github.io/nes_ebook)

I am following the afformentioned documents in order to learn about NES and Rust. This research wouldn't have come to fruition without these.