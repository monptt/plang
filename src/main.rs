mod interpreter;

fn main(){
    println!("Hello World!");
    println!("{}", interpreter::interpret("10"));
}
