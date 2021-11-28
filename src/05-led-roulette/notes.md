Start OpenCD
  - `cd /tmp`
  - `openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg`

Run
  - `cargo run`

Build
`cargo build --target thumbv7em-none-eabihf`

Size
`cargo size --target thumbv7em-none-eabihf --bin led-roulette --release -- -A`

Helpful GDB commands
  - `(gdb) target remote :3333`
  - `(gdb) set print asm-demangle on`
  - `(gdb) disassemble /m`
  - `(gdb) info locals`
  - `(gdb) layout src`
  - `(gdb) monitor reset halt`
