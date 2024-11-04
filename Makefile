tests:
	cargo build
	cd test-suite/test/core && python3 run.py --wasm ../../../target/debug/sasm > ../../../dump.tests 2>&1

get-tests:
	git clone https://github.com/WebAssembly/spec.git test-suite
	rm -rf test-suite/.git