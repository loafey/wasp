FILES = $(wildcard test-suite/test/core/*.wast)
targets = $(notdir $(basename $(FILES)))
$(targets): 
	watchexec -e rs "cargo run -- test-suite/test/core/$@.wast"

SIMD_FILES = $(wildcard test-suite/test/core/simd/*.wast)
simd_targets = $(notdir $(basename $(SIMD_FILES)))
$(simd_targets): 
	watchexec -e rs "cargo run -- test-suite/test/core/simd/$@.wast"

test-all:
	@cargo build
	@cd test-suite/test/core && (python3 run.py --wasm ../../../target/debug/wasp || true)> ../../../build/dump.tests 2>&1
	@zsh build/check.sh
test-setup:
	rm -rf test-suite/ || true
	git clone https://github.com/WebAssembly/spec.git test-suite
	rm -rf test-suite/.git