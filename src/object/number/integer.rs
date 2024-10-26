use super::operation;
use super::{super::object::ObjectTrait, value::Value};
use std::cmp;
use std::fmt;
use std::ops;

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

    pub fn gcd(a: Integer, b: Integer) -> Integer {
        if a.value < b.value {
            return Self::gcd(b, a);
        } else {
            if b.value == 0 {
                return a;
            } else {
                return Self::gcd(b, a % b);
            }
        }
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
    fn add(self, x: Self) -> Integer {
        return Integer {
            value: self.value + x.value,
        };
    }
}

impl operation::Sub for Integer {
    fn sub(self, x: Self) -> Integer {
        return Integer {
            value: self.value - x.value,
        };
    }
}

impl operation::Mul for Integer {
    fn mul(self, x: Self) -> Integer {
        return Integer {
            value: self.value * x.value,
        };
    }
}

impl operation::Div for Integer {
    fn div(self, x: Self) -> Integer {
        return Integer {
            value: self.value / x.value,
        };
    }
}

impl ops::Add<Integer> for Integer {
    type Output = Integer;
    fn add(self, rhs: Integer) -> Integer {
        return Integer {
            value: self.value + rhs.value,
        };
    }
}

impl ops::Sub<Integer> for Integer {
    type Output = Integer;
    fn sub(self, rhs: Integer) -> Integer {
        return Integer {
            value: self.value - rhs.value,
        };
    }
}

impl ops::Mul<Integer> for Integer {
    type Output = Integer;
    fn mul(self, rhs: Integer) -> Integer {
        return Integer {
            value: self.value * rhs.value,
        };
    }
}

impl ops::Div<Integer> for Integer {
    type Output = Integer;
    fn div(self, rhs: Integer) -> Integer {
        return Integer {
            value: self.value / rhs.value,
        };
    }
}

impl ops::Rem for Integer {
    type Output = Integer;
    fn rem(self, rhs: Self) -> Self::Output {
        return Integer {
            value: self.value % rhs.value,
        };
    }
}

// 等価演算子
impl cmp::PartialEq for Integer {
    fn eq(&self, other: &Self) -> bool {
        return self.value == other.value;
    }
}
#[cfg(test)]
mod tests {
    use crate::object::number::integer::Integer;

    #[test]
    fn test_integer_eq() {
        assert!(Integer { value: 1 } == Integer { value: 1 });
    }
}

impl From<i32> for Integer {
    fn from(value: i32) -> Self {
        return Integer { value: value };
    }
}
