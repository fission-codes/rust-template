#![cfg(target_arch = "wasm32")]

//! Test suite for the Web and headless browsers.
{% if node-or-web == "nodejs" %}
use wasm_bindgen_test::wasm_bindgen_test;
{% else %}
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
wasm_bindgen_test_configure!(run_in_browser);
{% endif %}
#[wasm_bindgen_test]
fn test_add() {
    assert_eq!({{crate_name}}_wasm::add(3, 2), 5);
    {{crate_name}}_wasm::console_log!("{}", "Test passes!");
}
