use super::integer::Integer;

use std::ops;

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

impl Value for RationalNumber {}

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
    fn mul(self, x: Self) -> Self {
        return RationalNumber {
            numerator: self.numerator.mul(x.numerator),
            denominator: self.denominator.mul(x.denominator),
        };
    }
}

impl operation::Div for RationalNumber {
    fn div(self, x: Self) -> Self {
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

impl ops::Add<RationalNumber> for RationalNumber {
    type Output = RationalNumber;
    fn add(self, rhs: RationalNumber) -> RationalNumber {
        return RationalNumber {
            numerator: self.numerator * rhs.denominator + rhs.numerator * self.denominator,
            denominator: self.denominator * rhs.denominator,
        };
    }
}

impl ops::Sub<RationalNumber> for RationalNumber {
    type Output = RationalNumber;
    fn sub(self, rhs: RationalNumber) -> RationalNumber {
        return RationalNumber {
            numerator: self.numerator * rhs.denominator - rhs.numerator * self.denominator,
            denominator: self.denominator * rhs.denominator,
        };
    }
}

impl ops::Mul<RationalNumber> for RationalNumber {
    type Output = RationalNumber;
    fn mul(self, x: Self) -> Self {
        return RationalNumber {
            numerator: self.numerator * x.numerator,
            denominator: self.denominator * x.denominator,
        };
    }
}

impl ops::Div<RationalNumber> for RationalNumber {
    type Output = RationalNumber;
    fn div(self, x: Self) -> Self {
        return RationalNumber {
            numerator: self.numerator * x.denominator,
            denominator: self.denominator * x.numerator,
        };
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
