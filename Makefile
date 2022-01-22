release:
	cargo build --release --offline

debug: 
	cargo build --offline

run_debug: debug
	./target/debug/my-redis

run_release: release
	./target/release/my-redis

run_debug_example: 
	cargo run --offline --example ${example_name}

run_release_example: 
	cargo run --offline --release --example ${example_name}

clean: 
	trash target; \
	trash Cargo.lock
