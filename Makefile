RUST_LIB_PATH = geodude-packet-parser/target/debug/libparser.a

build_c: rust_lib
	gcc c/main.c $(RUST_LIB_PATH) -o c/output && c/output

rust_lib:
	cd geodude-packet-parser; cargo build

clean:
	rm -r geodude-packet-parser/target
