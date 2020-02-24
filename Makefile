RUST_PKG_PATH = geodude-packet-parser/pkg
RUST_LIB_PATH = geodude-packet-parser/target/debug/libparser.a

.PHONY: run_c run_js clean
run_c: rust_lib
	gcc c/main.c $(RUST_LIB_PATH) -o c/output && c/output

run_js: $(RUST_PKG_PATH)
	cd javascript; node index.js

$(RUST_PKG_PATH):
	cd geodude-packet-parser; wasm-pack build --target nodejs

rust_lib:
	cd geodude-packet-parser; cargo build

clean:
	rm -r $(RUST_PKG_PATH)
	rm -r geodude-packet-parser/target
