use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_rayon::{init_thread_pool};
use js_sys::Promise;

#[wasm_bindgen]
pub fn xxx(n: usize) -> Promise {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    init_thread_pool(n)
}

#[wasm_bindgen]
pub fn contribute_wasm(input: &str) -> String {
    let response = contribute_with_string(input.to_string()).unwrap();
    return format!("{}", response);
}