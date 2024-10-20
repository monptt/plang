use super::integer::Integer;

use super::operation;
use super::operation::Add;
use super::operation::Sub;
use super::operation::Mul;
use super::operation::Div;

use super::value::Value;
use std::fmt;

#[derive(Clone, Copy)]
pub struct RationalNumber {
    numerator: Integer,
    denominator: Integer,
}

impl Value for RationalNumber {}

impl operation::Add for RationalNumber {
    fn add(&self, x: Self) -> Self {
        return RationalNumber {
            numerator: self.numerator.mul(x.denominator).add(x.numerator.mul(self.denominator)),
            denominator: self.denominator.mul(x.denominator)
        };
    }
}

impl operation::Sub for RationalNumber {
    fn sub(&self, x: Self) -> Self {
        return RationalNumber {
            numerator: self.numerator.mul(x.denominator).sub(x.numerator.mul(self.denominator)),
            denominator: self.denominator.mul(x.denominator)
        }
    }
}

impl operation::Mul for RationalNumber {
    fn mul(&self, x: Self) -> Self {
        return RationalNumber {
            numerator: self.numerator.mul(x.numerator),
            denominator: self.denominator.mul(x.denominator),
        };
    }
}

impl operation::Div for RationalNumber {
    fn div(&self, x: Self) -> Self {
        return RationalNumber {
            numerator: self.numerator.mul(x.denominator),
            denominator: self.denominator.mul(x.numerator),
        };
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

impl fmt::Display for RationalNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.denominator.value == 1 {
            return write!(f, "{}", self.numerator);
        }else {
            return write!(f, "\\frac{{ {} }}{{ {} }}", self.numerator, self.denominator);
        }
    }
}
