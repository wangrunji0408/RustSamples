LIB := target/debug/libc_call_rust.a
SRC := src/main.c
BIN := target/main

all: $(BIN)

run: $(BIN)
	$(BIN)

$(BIN): $(SRC) $(LIB)
	gcc -Ltarget/debug -lc_call_rust $(SRC) -o $(BIN)

$(LIB): src/lib.rs
	rustup override set nightly
	cargo build