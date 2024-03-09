use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hello() -> String {
    "Rusty Hello!".to_owned()
}

#[wasm_bindgen]
pub fn adder(first_value: u32, second_value: u32) -> Option<u32> {
    // checked_add is same as regular addition only that it wraps around in case of overflow.
    first_value.checked_add(second_value)
}
