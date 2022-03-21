o:
	cat input.txt | cargo run
	
b:
	hyperfine cargo run >> benchmark.txt
p:
	cargo flamegraph --dev

a:
	cargo asm --rust leetcode::main

m:
	RUSTFLAGS="--emit miri" cargo run
	# cargo +nightly miri run

