mod interpreter;
pub mod object;

use crate::interpreter::interpreter::Interpreter;

fn main(){
    let mut interpreter = Interpreter::new();
    println!("{}", interpreter.interpret("let x = 1 + 2 * 3\neval x"));
}
