tests:
	@cargo build
	@cd test-suite/test/core && (python3 run.py --wasm ../../../target/debug/sasm || true)> ../../../dump.tests 2>&1
	@zsh check.sh
get-tests:
	git clone https://github.com/WebAssembly/spec.git test-suite
	rm -rf test-suite/.git
