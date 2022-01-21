release:
	cargo build --release --offline

debug: 
	cargo build --offline

run_debug: debug
	./target/debug/my-redis

run_release: release
	./target/release/my-redis

clean: 
	trash target; \
	trash Cargo.lock
