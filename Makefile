RUSTC=/Users/zolkko/rust/bin/rustc
RUST_LIB=/Users/zolkko/rust/lib

all:
	DYLD_LIBRARY_PATH=${RUST_LIB}:${DYLD_LIBRARY_PATH} ${RUSTC} ./src/main.rs -g -o ./blumenplace-border

clean:
	unlink ./blumenplace-border
	rm -R ./blumenplace-border.dSYM

