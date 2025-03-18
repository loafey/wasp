tests:
	@cargo build
	@cd test-suite/test/core && (python3 run.py --wasm ../../../target/debug/wasp || true)> ../../../build/dump.tests 2>&1
	@zsh build/check.sh
get-tests:
	rm -rf test-suite/ || true
	git clone https://github.com/WebAssembly/spec.git test-suite
	rm -rf test-suite/.git

watch:
	watchexec -e rs "cargo run -- $(FILE)"