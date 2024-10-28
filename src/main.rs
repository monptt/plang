mod interpreter;
pub mod object;

use crate::interpreter::interpreter::Interpreter;

fn main(){
    let mut interpreter = Interpreter::new();
    println!("{}", interpreter.interpret("vec x = [2,4,3]"));
}
