RUST_PKG_PATH = lib-parser/pkg
RUST_LIB_PATH = geodude-packet-parser-c/target/debug/libparser.a

.PHONY: run_c run_js run_rust clean

run_rust:
	cd ; cargo test

run_c: rust_lib
	gcc c/main.c $(RUST_LIB_PATH) -o c/output && c/output

run_js: $(RUST_PKG_PATH)
	cd javascript; node index.js

$(RUST_PKG_PATH):
	cd geodude-packet-parser-js; wasm-pack build --target nodejs

rust_lib:
	cd geodude-packet-parser-c; cargo build

clean:
	rm -r $(RUST_PKG_PATH)
