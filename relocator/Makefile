@.PHONY: all

all: src/main.rs
	cargo +nightly build --release
	cp target/release/relocator ../compiler/

clean:
	cargo +nightly clean --release