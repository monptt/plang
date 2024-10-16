mod interpreter;
mod object;

fn main(){
    println!("{}", interpreter::interpreter::interpret("let x = 1\neval x"));
}
