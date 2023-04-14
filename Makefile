prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cd contract && cargo build --release --target wasm32-unknown-unknown
	cd counter-call && cargo build --release --target wasm32-unknown-unknown
	cd erc20/erc20-token && cargo build --release --target wasm32-unknown-unknown

	wasm-strip contract/target/wasm32-unknown-unknown/release/bridge_pool.wasm 2>/dev/null | true
	wasm-strip counter-call/target/wasm32-unknown-unknown/release/counter-call.wasm 2>/dev/null | true

test-only:
	cd tests && cargo test

test: build-contract
	mkdir -p tests/wasm
	cp contract/target/wasm32-unknown-unknown/release/bridge_pool.wasm tests/wasm
	cp counter-call/target/wasm32-unknown-unknown/release/counter-call.wasm tests/wasm
	cp erc20/target/wasm32-unknown-unknown/release/erc20_token.wasm tests/wasm/erc20.wasm
	cd tests && cargo test
	
clippy:
	cd contract && cargo clippy --all-targets -- -D warnings

check-lint: clippy
	cd contract && cargo fmt -- --check
	cd counter-call && cargo fmt -- --check
	cd tests && cargo fmt -- --check

lint: clippy
	cd contract && cargo fmt
	cd counter-call && cargo fmt
	cd tests && cargo fmt

clean:
	cd contract && cargo clean
	cd counter-call && cargo clean
	cd tests && cargo clean
	rm -rf tests/wasm
