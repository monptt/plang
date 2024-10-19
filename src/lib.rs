use wasm_bindgen::prelude::*;
mod interpreter;
mod object;

use crate::interpreter::interpreter::Interpreter;


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
    let mut interpreter = Interpreter::new();
    return interpreter.interpret(code);
}