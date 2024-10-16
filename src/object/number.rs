#[derive(Copy, Clone)]
pub struct Number{
    value: i32
}

impl Number{
    pub fn new(num: &str) -> Number{
        return Number{value: num.parse().unwrap()};
    }

    pub fn to_string(&self) -> String{
        return String::from(format!("{}", self.value));
    }
}
