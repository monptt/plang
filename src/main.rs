mod interpreter;

fn main(){
    println!("{}", interpreter::interpret("let x = 1\ny = x - 3"));
}
