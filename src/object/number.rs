use std::fmt;

use super::object::ObjectTrait;

#[derive(Copy, Clone)]
pub struct Number{
    value: i32
}

impl Number{
    pub fn new(num: &str) -> Number{
        return Number{value: num.parse().unwrap()};
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.value);
    }
}

impl ObjectTrait for Number{
    
}
