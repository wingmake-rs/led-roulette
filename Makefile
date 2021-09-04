reload:
	cargo build
	cargo run -- -q -ex 'target remote :3333' -ex 'load' -ex 'set print asm-demangle on' -ex 'set style sources off' -ex 'b main' -ex 'c' -ex 'layout split' target/thumbv7em-none-eabihf/debug/led-roulette
