# tui-calculator

**tui-calculator** is an advanced calculator written in Rust. It has two modes:

- Calculator (_simple calculator with history_)
- Programmer (_Allows you to convert numbers from one number system to another_)

## Demo (asciinema)

<a href="https://asciinema.org/a/704prGKcRZiBh9U45nRiFuFlC"><img src="https://asciinema.org/a/704prGKcRZiBh9U45nRiFuFlC.png"></a>

## Installation

- Download from releases
- Build from source: `cargo build --release`

## Usage

- Press `q` to quit
- Press `up arrow` to scroll through the modes up
- Press `down arrow` to scroll through the modes down

### Calculator

- Everything you write will be in the input
- Press `up arrow` to scroll through the history up
- Press `down arrow` to scroll through the history down
- Press `DEL` to clear history

### Programmer

- Everything you write will be in the input

#### Help

**Add "TYPE/type:number" to the beginning to determine the type of number
(without this, it is determined automatically); Examples:**

- HEX: `ABCDEF`
- DEC: `10`
- OCT: `7`
- BIN: `1010`
