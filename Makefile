release:
	cargo build --release --offline

debug: 
	cargo build --offline

server_debug: debug
	./target/debug/server

client_debug: debug
	./target/debug/client

server_release: release
	./target/release/server

client_release: release
	./target/drelease/client

bin: release
	./target/release/${bin}

bin_debug: debug
	./target/debug/${bin}

run_example: 
	cargo run --offline --release --example ${example}

run_debug_example: 
	cargo run --offline --example ${example}

clean: 
	trash target; \
	trash Cargo.lock
