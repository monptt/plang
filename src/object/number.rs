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

    pub fn add(a: Number, b: Number) -> Number{
        return Number{value: a.value + b.value};
    }

    pub fn sub(a: Number, b: Number) -> Number{
        return Number{value: a.value - b.value};
    }

    pub fn mul(a: Number, b: Number) -> Number{
        return Number{value: a.value * b.value};
    }

    pub fn div(a: Number, b: Number) -> Number{
        return Number{value: a.value / b.value};
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.value);
    }
}

impl ObjectTrait for Number{

}
