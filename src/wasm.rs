use js_sys::Promise;
use std::panic;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_rayon::init_thread_pool;

#[wasm_bindgen]
pub fn init_threads(n: usize) -> Promise {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    init_thread_pool(n)
}

#[wasm_bindgen]
pub fn contribute(entropy: &str, input: &str) -> String {
    let entropy = hex::decode(entropy).expect("Invalid entropy.");
    let mut buf = [0u8; 32];
    buf.copy_from_slice(&entropy);

    let response = crate::contribute_with_string(buf, input.to_string()).expect("Contribution failed.");
    return format!("{}", response);
}

#[wasm_bindgen]
pub fn hash_entropy(input: &str) -> String {
    let response = crate::hash_entropy(input.as_bytes()).unwrap();
    return format!("{}", hex::encode(response));
}
