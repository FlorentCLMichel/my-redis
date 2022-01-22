release:
	cargo build --release --offline

debug: 
	cargo build --offline

server_debug: debug
	./target/debug/server

client_debug: debug
	./target/debug/client

server_release: release
	./target/debug/server

client_release: release
	./target/debug/client

run_debug_example: 
	cargo run --offline --example ${example_name}

run_release_example: 
	cargo run --offline --release --example ${example_name}

clean: 
	trash target; \
	trash Cargo.lock
