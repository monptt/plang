use core::fmt;

use crate::object::number::{integer::Integer, rational_number::RationalNumber};

use super::function::{Function, FunctionTrait};

#[derive(Clone)]
pub struct Monomial {
    pub coefficient: RationalNumber, // 係数
    pub degree: Integer,             // 次数
}

impl Monomial {
    pub fn new(coefficient: RationalNumber, degree: Integer) -> Monomial {
        return Monomial {
            coefficient: coefficient,
            degree: degree,
        };
    }
}

impl FunctionTrait for Monomial {
    fn calc(&self, x: RationalNumber) -> RationalNumber {
        let mut ret_value = self.coefficient;

        for _ in 0..self.degree.value {
            ret_value = ret_value * x;
        }

        return ret_value;
    }
}

impl fmt::Display for Monomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.degree == Integer::from(0) {
            return write!(f, "{}", self.coefficient);
        } else if self.degree == Integer::from(1) {
            if self.coefficient == RationalNumber::from(1) {
                return write!(f, "x");
            } else {
                return write!(f, "{} x", self.coefficient);
            }
        } else {
            if self.coefficient == RationalNumber::from(1) {
                return write!(f, "x ^ {{ {} }}", self.degree);
            } else {
                return write!(f, "{} x ^ {{ {} }}", self.coefficient, self.degree);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::object::number::{integer::Integer, rational_number::RationalNumber};

    use super::FunctionTrait;
    use super::Monomial;

    #[test]
    fn test_monomial() {
        let monomial = Monomial::new(RationalNumber::from(3), Integer::from(2)); // 3x^2
        let expected = RationalNumber::from(48);
        assert!(monomial.calc(RationalNumber::from(4)) == expected);
    }
}
