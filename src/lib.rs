use wasm_bindgen::prelude::*;
mod interpreter;


#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}


#[wasm_bindgen]
pub fn interpret(code: &str) -> String {
    return interpreter::interpreter::interpret(code);
}