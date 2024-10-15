mod interpreter;

fn main(){
    println!("Hello World!");
    println!("{}", interpreter::interpret("hello\nworld"));
}
