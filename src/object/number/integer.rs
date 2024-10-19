use super::operation;
use super::{super::object::ObjectTrait, value::Value};
use std::fmt;

#[derive(Copy, Clone)]
pub struct Integer {
    pub value: i32,
}

impl Integer {
    pub fn new(num: &str) -> Integer {
        return Integer {
            value: num.parse().unwrap(),
        };
    }

    pub fn add(a: Integer, b: Integer) -> Integer {
        return Integer {
            value: a.value + b.value,
        };
    }

    pub fn sub(a: Integer, b: Integer) -> Integer {
        return Integer {
            value: a.value - b.value,
        };
    }

    pub fn mul(a: Integer, b: Integer) -> Integer {
        return Integer {
            value: a.value * b.value,
        };
    }

    pub fn div(a: Integer, b: Integer) -> Integer {
        return Integer {
            value: a.value / b.value,
        };
    }
}

impl ObjectTrait for Integer {
    fn to_string(&self) -> String {
        let value = self.value;
        let str = value.to_string();
        return str;
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.value);
    }
}

impl Value for Integer {}

impl operation::Add for Integer {
    fn add(&self, x :Self) -> Box<dyn Value> {
        return Box::new(Integer { value: self.value + x.value });
    }
}

impl operation::Sub for Integer {
    fn sub(&self, x :Self) -> Box<dyn Value> {
        return Box::new(Integer { value: self.value - x.value });
    }
}

impl operation::Mul for Integer {
    fn mul(&self, x :Self) -> Box<dyn Value> {
        return Box::new(Integer { value: self.value * x.value });
    }
}

impl operation::Div for Integer {
    fn div(&self, x :Self) -> Box<dyn Value> {
        return Box::new(Integer { value: self.value / x.value });
    }
}
