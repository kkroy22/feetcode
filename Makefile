run:
	make fmt
	cat input.txt | cargo run
	
bench:
	hyperfine cargo run >> benchmark.txt

flame:
	cargo flamegraph --dev

asm:
	cargo asm --rust leetcode::main

miri:
	RUSTFLAGS="--emit miri" cargo run
	# cargo +nightly miri run

fmt:
	cargo fmt