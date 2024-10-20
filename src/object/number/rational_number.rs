use super::integer::Integer;
use super::operation;
use super::value::Value;
use std::fmt;

#[derive(Clone, Copy)]
pub struct RationalNumber {
    numerator: i32,
    denominator: i32,
}

impl Value for RationalNumber {}

impl operation::Add for RationalNumber {
    fn add(&self, x: Self) -> Self {
        return RationalNumber {
            numerator: self.numerator + x.numerator,
            denominator: 1,
        };
    }
}

impl operation::Sub for RationalNumber {
    fn sub(&self, x: Self) -> Self {
        return RationalNumber {
            numerator: self.numerator - x.numerator,
            denominator: 1,
        };
    }
}

impl operation::Mul for RationalNumber {
    fn mul(&self, x: Self) -> Self {
        return RationalNumber {
            numerator: self.numerator * x.numerator,
            denominator: 1,
        };
    }
}

impl operation::Div for RationalNumber {
    fn div(&self, x: Self) -> Self {
        return RationalNumber {
            numerator: self.numerator / x.numerator,
            denominator: self.numerator / x.numerator,
        };
    }
}

impl From<&Integer> for RationalNumber {
    fn from(n: &Integer) -> RationalNumber {
        return RationalNumber {
            numerator: n.value,
            denominator: 1,
        };
    }
}

impl fmt::Display for RationalNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if(self.denominator == 1){
            return write!(f, "{}", self.numerator);
        }else {
            return write!(f, "{}/{}", self.numerator, self.denominator);
        }
    }
}
