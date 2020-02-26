#[macro_use]
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

extern crate parser;
use parser::parse_packet;

#[wasm_bindgen]
pub fn js_parse_packet(bytes: &JsValue) -> JsValue {
    let bytes: Vec<u8> = bytes.into_serde().unwrap();
    let result = parse_packet(&bytes).unwrap();
    JsValue::from_serde(&result).unwrap()
}

