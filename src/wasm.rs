use js_sys::Promise;
use std::panic;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_rayon::init_thread_pool;

use crate::contribute_with_json_string;

#[wasm_bindgen]
pub fn init_threads(n: usize) -> Promise {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    init_thread_pool(n)
}

#[wasm_bindgen]
pub fn contribute(entropy: &str, input: &str) -> String {
    let mut buf = [0u8; 32];
    buf.copy_from_slice(&hex::decode(entropy).unwrap());
    let response = contribute_with_json_string(buf, input.to_string()).unwrap();
    return format!("{}", response);
}

#[wasm_bindgen]
pub fn get_entropy(input: &str) -> String {
    let response = crate::get_entropy(input.as_bytes());
    return format!("{}", hex::encode(response));
}
