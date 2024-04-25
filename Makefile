all: 
	@cargo run #2>/dev/null //descomentar esta parte para que no haga output del debugeo

ex00: 
	@cargo run --bin ex00 #2>/dev/null

ex01: 
	@cargo run --bin ex01 #2>/dev/null

optimize:
	cargo run -- opt-level="3"

build: 
	cargo build

clean:
	rm -rf target

.PHONY: library ex00 ex01 build clean