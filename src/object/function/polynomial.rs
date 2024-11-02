use crate::object::number::{integer::Integer, rational_number::RationalNumber};

use super::{function::Function, monomial::Monomial};

// 多項式関数
struct Polynomial {
    coefficients: Vec<RationalNumber>,
}

impl Function for Polynomial {
    fn map(&self, x: RationalNumber) -> RationalNumber {
        let mut ret_value = RationalNumber::from(0);

        let mut temp_pow = RationalNumber::from(1);

        for coefficient in &self.coefficients {
            ret_value = ret_value + *coefficient * temp_pow;
            temp_pow = temp_pow * x;
        }

        return ret_value;
    }
}

impl Polynomial {
    fn new(coefficients: Vec<RationalNumber>) -> Polynomial {
        return Polynomial {
            coefficients: coefficients,
        };
    }
}

impl From<&Monomial> for Polynomial {
    fn from(monomial: &Monomial) -> Polynomial {
        let mut coefficients: Vec<RationalNumber> = Vec::new();
        for i in 0..monomial.degree.value + 1 {
            if i == monomial.degree.value {
                coefficients.push(monomial.coefficient);
            } else {
                coefficients.push(RationalNumber::from(0));
            }
        }

        return Polynomial::new(coefficients);
    }
}
