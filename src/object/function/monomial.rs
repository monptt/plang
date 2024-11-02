use crate::object::number::{integer::Integer, rational_number::RationalNumber};

use super::function::Function;

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

impl Function for Monomial {
    fn calc(&self, x: RationalNumber) -> RationalNumber {
        let mut ret_value = self.coefficient;

        for _ in 0..self.degree.value {
            ret_value = ret_value * x;
        }

        return ret_value;
    }
}

#[cfg(test)]
mod tests {
    use crate::object::number::{integer::Integer, rational_number::RationalNumber};

    use super::Monomial;
    use super::Function;

    #[test]
    fn test_monomial() {
        let monomial = Monomial::new(RationalNumber::from(3), Integer::from(2)); // 3x^2
        let expected = RationalNumber::from(48);
        assert!(monomial.calc(RationalNumber::from(4)) == expected);
    }
}
