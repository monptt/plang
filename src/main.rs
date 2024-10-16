mod interpreter;
pub mod object;

fn main(){
    println!("{}", interpreter::interpreter::interpret("let x = 1 + 2\neval x"));
}
