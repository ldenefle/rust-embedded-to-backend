# Description

Talk about building a parser library exportable for embedded C and compilable to web assembly to be used on backend side to

# Dependencies 

The project makes heavy use of:
+ [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) ecosystem
+ [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
+ node 
+ gcc

You might want to install wasm23 toolchain with rustup
```
rustup target add wasm32-unknown-unknown
```

# Commands

```
make run_rust
```
```
make run_c
```
```
make run_js
```
