use super::integer::Integer;

use std::cmp;
use std::ops;
use std::result;

use super::operation;
use super::operation::Add;
use super::operation::Div;
use super::operation::Mul;
use super::operation::Sub;

use super::value::Value;
use std::fmt;

#[derive(Clone, Copy)]
pub struct RationalNumber {
    numerator: Integer,
    denominator: Integer,
}

impl RationalNumber {
    fn new(a: Integer, b: Integer) -> RationalNumber {
        return RationalNumber {
            numerator: a,
            denominator: b,
        };
    }

    fn reduce(x: &RationalNumber) -> RationalNumber {
        let gcd = Integer::gcd(x.numerator, x.denominator);
        return RationalNumber {
            numerator: x.numerator / gcd,
            denominator: x.denominator / gcd,
        };
    }
}

impl operation::Add for RationalNumber {
    fn add(self, rhs: Self) -> Self {
        return self + rhs;
    }
}

impl operation::Sub for RationalNumber {
    fn sub(self, rhs: Self) -> Self {
        return self - rhs;
    }
}

impl operation::Mul for RationalNumber {
    fn mul(self, rhs: Self) -> Self {
        return self * rhs;
    }
}

impl operation::Div for RationalNumber {
    fn div(self, rhs: Self) -> Self {
        return self / rhs;
    }
}

impl From<&Integer> for RationalNumber {
    fn from(n: &Integer) -> RationalNumber {
        return RationalNumber {
            numerator: *n,
            denominator: Integer::from(1),
        };
    }
}

impl From<i32> for RationalNumber {
    fn from(x: i32) -> RationalNumber {
        return RationalNumber {
            numerator: Integer::from(x),
            denominator: Integer::from(1)
        }
    }
}

impl From<&String> for RationalNumber {
    fn from(in_str: &String) -> Self {
        let mut numerator: Integer = Integer::from(0);
        let mut denominator: Integer = Integer::from(1);

        // 小数点があったか
        let mut is_decimal = false;


        for c in in_str.chars() {
            if "0123456789".contains(c) {
                let digit: i32 = c.to_string().parse().unwrap();
                numerator = numerator * Integer::from(10);
                numerator = numerator + Integer::from(digit);

                if is_decimal {
                    denominator = denominator * Integer::from(10);
                }
            }else if c == '.' {
                is_decimal = true;
            }
        }

        return RationalNumber{
            numerator: numerator,
            denominator: denominator
        };
    }
}

impl ops::Add<RationalNumber> for RationalNumber {
    type Output = RationalNumber;
    fn add(self, rhs: RationalNumber) -> RationalNumber {
        let result = RationalNumber {
            numerator: self.numerator * rhs.denominator + rhs.numerator * self.denominator,
            denominator: self.denominator * rhs.denominator,
        };
        return RationalNumber::reduce(&result);
    }
}

impl ops::Sub<RationalNumber> for RationalNumber {
    type Output = RationalNumber;
    fn sub(self, rhs: RationalNumber) -> RationalNumber {
        let result = RationalNumber {
            numerator: self.numerator * rhs.denominator - rhs.numerator * self.denominator,
            denominator: self.denominator * rhs.denominator,
        };
        return RationalNumber::reduce(&result);
    }
}

impl ops::Mul<RationalNumber> for RationalNumber {
    type Output = RationalNumber;
    fn mul(self, x: Self) -> Self {
        let result = RationalNumber {
            numerator: self.numerator * x.numerator,
            denominator: self.denominator * x.denominator,
        };
        return RationalNumber::reduce(&result);
    }
}

impl ops::Div<RationalNumber> for RationalNumber {
    type Output = RationalNumber;
    fn div(self, x: Self) -> Self {
        let result = RationalNumber {
            numerator: self.numerator * x.denominator,
            denominator: self.denominator * x.numerator,
        };
        return RationalNumber::reduce(&result);
    }
}

// 等価演算子
impl cmp::PartialEq for RationalNumber {
    fn eq(&self, other: &Self) -> bool {
        return self.numerator * other.denominator == other.numerator * self.denominator;
    }
}
#[cfg(test)]
mod tests {
    use crate::object::number::{integer::Integer, rational_number::RationalNumber};

    #[test]
    fn test_rational_number_eq() {
        assert!(RationalNumber::new(Integer::from(1), Integer::from(2)) == RationalNumber::new(Integer::from(2), Integer::from(4)));
    }
}

impl fmt::Display for RationalNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.denominator.value == 1 {
            return write!(f, "{}", self.numerator);
        } else {
            return write!(
                f,
                "\\frac{{ {} }}{{ {} }}",
                self.numerator, self.denominator
            );
        }
    }
}
